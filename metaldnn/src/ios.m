#import <QuartzCore/QuartzCore.h>
#import <UIKit/UIKit.h>
#import <objc/runtime.h>

@interface MyView : UIView

@end

@implementation MyView
+ (Class)layerClass {
  return [CAMetalLayer class];
}
@end

CAMetalLayer *layer_init(void *window_view, id<MTLDevice> device) {
  UIView *view = (UIView *)window_view;
  CAMetalLayer *metal_layer = (CAMetalLayer *)view.layer;
  return metal_layer;
}
