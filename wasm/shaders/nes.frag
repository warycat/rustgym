#version 300 es
precision highp float;

in vec2 v_tex_coord;
out vec4 FragColor;
uniform sampler2D u_texture;

void main() {
  // uvec2 coord = uvec2(v_tex_coord);
  // uint i = coord[1] * 8u + coord[0];
  // vec4 i_color = vec4(u_rgbav[i * 4u], u_rgbav[i * 4u + 1u],
  //                     u_rgbav[i * 4u + 2u], u_rgbav[i * 4u + 3u]);
  FragColor = texture(u_texture, v_tex_coord);
}
