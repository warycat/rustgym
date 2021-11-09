#include <metal_stdlib>
#include <simd/simd.h>
using namespace metal;

typedef struct {
  float4 position [[position]];
  float2 texture_coord;
} Vertex;

vertex Vertex quad_vertex(uint vertex_id [[vertex_id]],
                          constant Vertex *vert_array [[buffer(0)]],
                          constant float2 *viewport_size_ptr [[buffer(1)]]) {
  Vertex out;

  float2 pixel_space_pos = vert_array[vertex_id].position.xy;
  float2 viewport_size = *viewport_size_ptr;

  float2 clip_space_pos = (pixel_space_pos / viewport_size) * 2.0;

  out.position = float4(clip_space_pos, 0.0, 1.0);
  out.texture_coord = vert_array[vertex_id].texture_coord;

  return out;
}

fragment float4 sampling_shader(Vertex in [[stage_in]],
                                texture2d<half> color_texture [[texture(0)]]) {
  constexpr sampler texture_sampler(mag_filter::linear, min_filter::linear);

  const half4 color_sample =
      color_texture.sample(texture_sampler, in.texture_coord);

  return float4(color_sample);
}
