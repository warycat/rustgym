#version 300 es

in vec2 a_position;
uniform vec2 u_resolution;

void main() {
  vec2 zero_to_one = a_position / u_resolution;
  vec2 zero_to_two = zero_to_one * 2.0;
  vec2 clip_space = zero_to_two - 1.0;
  gl_Position = vec4(clip_space * vec2(1, -1), 0, 1);
}