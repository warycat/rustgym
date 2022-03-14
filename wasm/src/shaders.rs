use log::{error, info};
use web_sys::{
    HtmlCanvasElement, Request, RequestInit, WebGl2RenderingContext, WebGlContextAttributes,
    WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture, WebGlUniformLocation,
};

pub const VERTEX_SHADER: u32 = WebGlRenderingContext::VERTEX_SHADER;
pub const FRAGMENT_SHADER: u32 = WebGlRenderingContext::FRAGMENT_SHADER;
pub const ARRAY_BUFFER: u32 = WebGlRenderingContext::ARRAY_BUFFER;
pub const STATIC_DRAW: u32 = WebGlRenderingContext::STATIC_DRAW;
pub const UNSIGNED_BYTE: u32 = WebGlRenderingContext::UNSIGNED_BYTE;
pub const FLOAT: u32 = WebGlRenderingContext::FLOAT;
pub const UNSIGNED_INT: u32 = WebGlRenderingContext::UNSIGNED_INT;
pub const COLOR_BUFFER_BIT: u32 = WebGlRenderingContext::COLOR_BUFFER_BIT;
pub const LINK_STATUS: u32 = WebGlRenderingContext::LINK_STATUS;
pub const COMPILE_STATUS: u32 = WebGlRenderingContext::COMPILE_STATUS;
pub const TRIANGLE_STRIP: u32 = WebGlRenderingContext::TRIANGLE_STRIP;
pub const TEXTURE_2D: u32 = WebGlRenderingContext::TEXTURE_2D;
pub const _AUDIO_SAMPLE_RATE: u32 = 44100;
pub const RGBA: u32 = WebGlRenderingContext::RGBA;
pub const TEXTURE0: u32 = WebGlRenderingContext::TEXTURE0;
pub const TEXTURE_MAG_FILTER: u32 = WebGlRenderingContext::TEXTURE_MAG_FILTER;
pub const LINEAR: u32 = WebGlRenderingContext::LINEAR;
pub const NEAREST: u32 = WebGlRenderingContext::NEAREST;
pub const TEXTURE_MIN_FILTER: u32 = WebGlRenderingContext::TEXTURE_MIN_FILTER;
pub const TEXTURE_WRAP_S: u32 = WebGlRenderingContext::TEXTURE_WRAP_S;
pub const CLAMP_TO_EDGE: u32 = WebGlRenderingContext::CLAMP_TO_EDGE;
pub const TEXTURE_WRAP_T: u32 = WebGlRenderingContext::TEXTURE_WRAP_T;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: u32 =
    WebGl2RenderingContext::MAX_VERTEX_UNIFORM_COMPONENTS;
pub const MAX_TEXTURE_IMAGE_UNITS: u32 = WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS;

pub fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = ctx.create_shader(shader_type).unwrap();
    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);
    let compile_status = ctx
        .get_shader_parameter(&shader, COMPILE_STATUS)
        .as_bool()
        .unwrap();
    if compile_status {
        Ok(shader)
    } else {
        error!("{}", source);
        error!("{}", ctx.get_shader_info_log(&shader).unwrap());
        Err(ctx.get_shader_info_log(&shader).unwrap())
    }
}

pub fn link_program(
    ctx: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = ctx.create_program().unwrap();
    ctx.attach_shader(&program, vert_shader);
    ctx.attach_shader(&program, frag_shader);
    ctx.link_program(&program);
    let link_status = ctx
        .get_program_parameter(&program, LINK_STATUS)
        .as_bool()
        .unwrap();
    if link_status {
        Ok(program)
    } else {
        Err(ctx.get_program_info_log(&program).unwrap())
    }
}
