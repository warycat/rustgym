#import <AppKit/AppKit.h>
#import <QuartzCore/QuartzCore.h>

CAMetalLayer *layer_init(void *window_view, id<MTLDevice> device) {
  NSView *view = (NSView *)window_view;
  view.wantsLayer = YES;
  view.layer = [CAMetalLayer layer];
  CAMetalLayer *metal_layer = (CAMetalLayer *)view.layer;
  metal_layer.device = device;
  NSLog(@"CAMetalLayer %@", metal_layer);
  return metal_layer;
}