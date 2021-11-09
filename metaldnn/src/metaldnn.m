#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
#import <MetalPerformanceShaders/MetalPerformanceShaders.h>
#import <QuartzCore/QuartzCore.h>
#import <dispatch/dispatch.h>

typedef struct {
  vector_float4 position;
  vector_float2 texture_coord;
} Vertex;

id<MTLDevice> device_init() {
  id<MTLDevice> device = MTLCreateSystemDefaultDevice();
  NSLog(@"MTLDevice %@", device);
  return device;
}

id<MTLLibrary> library_init(id<MTLDevice> device, const void *buffer,
                            size_t size) {
  dispatch_queue_main_t queue = dispatch_get_main_queue();
  dispatch_data_t data = dispatch_data_create(buffer, size, queue,
                                              DISPATCH_DATA_DESTRUCTOR_DEFAULT);
  NSError *error;
  id<MTLLibrary> library = [device newLibraryWithData:data error:&error];
  if (error) {
    NSLog(@"%@", error);
  }
  NSLog(@"MTLLibrary %@", library);
  return library;
}

id<MTLBuffer> vertex_buffer_init(id<MTLDevice> device) {
  static const Vertex vertices[] = {
      {.position = {-100.0, 100.0, 0, 1}, .texture_coord = {0.0, 0.0}},
      {.position = {100.0, 100.0, 0, 1}, .texture_coord = {1.0, 0.0}},
      {.position = {-100.0, -100.0, 0, 1}, .texture_coord = {0.0, 1.0}},
      {.position = {100.0, -100.0, 0, 1}, .texture_coord = {1.0, 1.0}}};
  id<MTLBuffer> vertex_buffer =
      [device newBufferWithBytes:vertices
                          length:sizeof(vertices)
                         options:MTLResourceOptionCPUCacheModeDefault];
  NSLog(@"MTLBuffer %@", vertex_buffer);
  return vertex_buffer;
}

id<MTLBuffer> viewport_buffer_init(id<MTLDevice> device) {
  static const vector_float2 viewport[] = {{800.0, 600.0}};
  id<MTLBuffer> viewport_buffer =
      [device newBufferWithBytes:viewport
                          length:sizeof(viewport)
                         options:MTLResourceOptionCPUCacheModeDefault];
  NSLog(@"MTLBuffer %@", viewport_buffer);
  return viewport_buffer;
}

id<MTLTexture> texture_init(id<MTLDevice> device, size_t width, size_t height,
                            BOOL grayscale) {
  MTLPixelFormat format = MTLPixelFormatRGBA8Unorm;
  if (grayscale) {
    format = MTLPixelFormatR8Unorm;
  }
  MTLTextureDescriptor *texture_desc =
      [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:format
                                                         width:width
                                                        height:height
                                                     mipmapped:NO];
  id<MTLTexture> texture = [device newTextureWithDescriptor:texture_desc];
  NSLog(@"MTLTexture %@", texture);
  return texture;
}

void texture_update(id<MTLTexture> texture, size_t width, size_t height,
                    const void *buffer, size_t pixel_size) {
  MTLRegion region = MTLRegionMake2D(0, 0, width, height);
  [texture replaceRegion:region
             mipmapLevel:0
               withBytes:buffer
             bytesPerRow:width * pixel_size];
}

id<MTLRenderPipelineState> pipeline_init(id<MTLDevice> device,
                                         id<MTLLibrary> library) {
  id<MTLFunction> vert_fn = [library newFunctionWithName:@"quad_vertex"];
  id<MTLFunction> frag_fn = [library newFunctionWithName:@"sampling_shader"];
  MTLRenderPipelineDescriptor *pl_desc =
      [[MTLRenderPipelineDescriptor alloc] init];
  pl_desc.colorAttachments[0].pixelFormat = MTLPixelFormatBGRA8Unorm;
  pl_desc.vertexFunction = vert_fn;
  pl_desc.fragmentFunction = frag_fn;
  NSError *error = nil;
  id<MTLRenderPipelineState> pipeline =
      [device newRenderPipelineStateWithDescriptor:pl_desc error:&error];
  if (error) {
    NSLog(@"%@", error);
  }
  [pl_desc release];
  NSLog(@"MTLRenderPipelineState %@", pipeline);
  return pipeline;
}

id<MTLCommandQueue> command_queue_init(id<MTLDevice> device) {
  id<MTLCommandQueue> command_queue = [device newCommandQueue];
  NSLog(@"MTLCommandQueue %@", command_queue);
  return command_queue;
}

void redraw(CAMetalLayer *layer, id<MTLRenderPipelineState> pipeline,
            id<MTLCommandQueue> command_queue, id<MTLBuffer> vertex_buffer,
            id<MTLBuffer> viewport_buffer, id<MTLTexture> texture) {
  id<CAMetalDrawable> drawable = [layer nextDrawable];
  if (drawable) {
    MTLRenderPassDescriptor *pass_desc =
        [MTLRenderPassDescriptor renderPassDescriptor];
    pass_desc.colorAttachments[0].texture = drawable.texture;
    pass_desc.colorAttachments[0].clearColor =
        MTLClearColorMake(0.85, 0.85, 0.85, 1);
    pass_desc.colorAttachments[0].storeAction = MTLStoreActionStore;
    pass_desc.colorAttachments[0].loadAction = MTLLoadActionClear;
    id<MTLCommandBuffer> command_buffer = [command_queue commandBuffer];
    id<MTLRenderCommandEncoder> command_encoder =
        [command_buffer renderCommandEncoderWithDescriptor:pass_desc];
    [command_encoder setRenderPipelineState:pipeline];
    [command_encoder setVertexBuffer:vertex_buffer offset:0 atIndex:0];
    [command_encoder setVertexBuffer:viewport_buffer offset:0 atIndex:1];
    [command_encoder setFragmentTexture:texture atIndex:0];
    [command_encoder drawPrimitives:MTLPrimitiveTypeTriangleStrip
                        vertexStart:0
                        vertexCount:4];
    [command_encoder endEncoding];
    [command_buffer presentDrawable:drawable];
    [command_buffer commit];
  }
}

///////////////////////////////////////////////////

bool device_available(id<MTLDevice> device) {
  return MPSSupportsMTLDevice(device);
}

void metaldnn_device_release(id<MTLDevice> device) {
  NSLog(@"device_release %@", device);
  [device release];
}

MPSImageBatch *input_init(id<MTLDevice> device, size_t batch, size_t width,
                          size_t height) {
  MPSImageDescriptor *image_descriptor = [MPSImageDescriptor
      imageDescriptorWithChannelFormat:MPSImageFeatureChannelFormatUnorm8
                                 width:width
                                height:height
                       featureChannels:1
                        numberOfImages:1
                                 usage:MTLTextureUsageShaderWrite |
                                       MTLTextureUsageShaderRead];
  NSMutableArray *image_batch = [[NSMutableArray alloc] init];
  for (size_t i = 0; i < batch; i++) {
    MPSImage *image = [[MPSImage alloc] initWithDevice:device
                                       imageDescriptor:image_descriptor];
    [image_batch addObject:image];
  }
  NSLog(@"input_init %@", image_batch);
  return image_batch;
}

void metaldnn_input_release(NSMutableArray *image_batch) {
  NSLog(@"input_release %@", image_batch);
  [image_batch release];
}

@interface MetaldnnPerson : NSObject {
  NSString *firstName;
  NSString *lastName;
}

@property(assign) NSString *firstName;
@property(assign) NSString *lastName;

- (void)dealloc;

@end

@implementation MetaldnnPerson
@synthesize firstName;
@synthesize lastName;

- (void)dealloc {
  NSLog(@"dealloc");
  [super dealloc];
}
@end

MetaldnnPerson *metaldnn_person(NSString *first, NSString *last) {
  MetaldnnPerson *p = [[MetaldnnPerson alloc] init];
  p.firstName = first;
  p.lastName = last;
  return p;
}
