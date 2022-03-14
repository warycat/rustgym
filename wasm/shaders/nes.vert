#version 300 es

in uvec2 a_position;
const vec2 u_resolution = vec2(256.0, 256.0);
out vec2 v_tex_coord;

void main() {
  vec2 a_position_f = vec2(a_position);
  vec2 zero_to_one = a_position_f / u_resolution;
  vec2 zero_to_two = zero_to_one * 2.0;
  vec2 clip_space = zero_to_two - 1.0;
  v_tex_coord = vec2(a_position_f) / u_resolution;
  gl_Position = vec4(clip_space * vec2(1, -1), 0, 1);
}