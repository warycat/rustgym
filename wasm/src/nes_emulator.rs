use crate::utils::*;
use log::{error, info};
use nes::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{
    HtmlCanvasElement, Request, RequestInit, WebGl2RenderingContext, WebGlContextAttributes,
    WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture, WebGlUniformLocation,
};

const VERTEX_SHADER: u32 = WebGlRenderingContext::VERTEX_SHADER;
const FRAGMENT_SHADER: u32 = WebGlRenderingContext::FRAGMENT_SHADER;
const ARRAY_BUFFER: u32 = WebGlRenderingContext::ARRAY_BUFFER;
const STATIC_DRAW: u32 = WebGlRenderingContext::STATIC_DRAW;
const UNSIGNED_BYTE: u32 = WebGlRenderingContext::UNSIGNED_BYTE;
const FLOAT: u32 = WebGlRenderingContext::FLOAT;
const UNSIGNED_INT: u32 = WebGlRenderingContext::UNSIGNED_INT;
const COLOR_BUFFER_BIT: u32 = WebGlRenderingContext::COLOR_BUFFER_BIT;
const LINK_STATUS: u32 = WebGlRenderingContext::LINK_STATUS;
const COMPILE_STATUS: u32 = WebGlRenderingContext::COMPILE_STATUS;
const TRIANGLE_STRIP: u32 = WebGlRenderingContext::TRIANGLE_STRIP;
const TEXTURE_2D: u32 = WebGlRenderingContext::TEXTURE_2D;
const _AUDIO_SAMPLE_RATE: u32 = 44100;
const RGBA: u32 = WebGlRenderingContext::RGBA;
const TEXTURE0: u32 = WebGlRenderingContext::TEXTURE0;
const TEXTURE_MAG_FILTER: u32 = WebGlRenderingContext::TEXTURE_MAG_FILTER;
const LINEAR: u32 = WebGlRenderingContext::LINEAR;
const NEAREST: u32 = WebGlRenderingContext::NEAREST;
const TEXTURE_MIN_FILTER: u32 = WebGlRenderingContext::TEXTURE_MIN_FILTER;
const TEXTURE_WRAP_S: u32 = WebGlRenderingContext::TEXTURE_WRAP_S;
const CLAMP_TO_EDGE: u32 = WebGlRenderingContext::CLAMP_TO_EDGE;
const TEXTURE_WRAP_T: u32 = WebGlRenderingContext::TEXTURE_WRAP_T;
const MAX_VERTEX_UNIFORM_COMPONENTS: u32 = WebGl2RenderingContext::MAX_VERTEX_UNIFORM_COMPONENTS;
const MAX_TEXTURE_IMAGE_UNITS: u32 = WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS;
fn compile_shader(
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

fn link_program(
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

struct BackgroundProgram {
    shader_program: WebGlProgram,
    a_position: u32,
    a_tex_coord: u32,
    u_rgbav: WebGlUniformLocation,
    texture: WebGlTexture,
}

fn init(
    canvas: &HtmlCanvasElement,
) -> Result<(WebGl2RenderingContext, BackgroundProgram), JsValue> {
    let mut attributes: WebGlContextAttributes = WebGlContextAttributes::default();
    attributes.antialias(false);
    let ctx: WebGl2RenderingContext = canvas
        .get_context_with_context_options("webgl2", &attributes)?
        .unwrap()
        .dyn_into()?;
    let attributes = ctx.get_context_attributes().unwrap();
    info!("{:?}", attributes);
    let nes_background_vert_shader = compile_shader(
        &ctx,
        VERTEX_SHADER,
        include_str!("../shaders/nes_background.vert"),
    )?;
    let nes_background_frag_shader = compile_shader(
        &ctx,
        FRAGMENT_SHADER,
        include_str!("../shaders/nes_background.frag"),
    )?;
    let shader_program = link_program(
        &ctx,
        &nes_background_vert_shader,
        &nes_background_frag_shader,
    )?;
    let a_position = ctx.get_attrib_location(&shader_program, "a_position") as u32;
    let a_tex_coord = ctx.get_attrib_location(&shader_program, "a_tex_coord") as u32;
    let u_rgbav = ctx
        .get_uniform_location(&shader_program, "u_rgbav")
        .unwrap();

    let texture: WebGlTexture = ctx.create_texture().unwrap();
    ctx.bind_texture(TEXTURE_2D, Some(&texture));
    let png_bytes = include_bytes!("../cubetexture.png");
    let decoder = png::Decoder::new(png_bytes.as_ref());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        TEXTURE_2D,
        0,
        RGBA as i32,
        256,
        256,
        0,
        RGBA,
        UNSIGNED_BYTE,
        Some(bytes),
    )
    .unwrap();
    ctx.tex_parameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
    ctx.tex_parameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
    // ctx.generate_mipmap(TEXTURE_2D);
    ctx.active_texture(TEXTURE0);

    let background_program = BackgroundProgram {
        shader_program,
        a_position,
        a_tex_coord,
        u_rgbav,
        texture,
    };

    Ok((ctx, background_program))
}

#[derive(new)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn draw_rect(ctx: &WebGl2RenderingContext, rect: Rect) {
    let positions = [
        rect.x,
        rect.y,
        0,
        0,
        rect.x + rect.width,
        rect.y,
        8,
        0,
        rect.x,
        rect.y + rect.height,
        0,
        8,
        rect.x + rect.width,
        rect.y + rect.height,
        8,
        8,
    ];
    let (_, bytes, _) = unsafe { positions.align_to::<u8>() };
    ctx.buffer_data_with_u8_array(ARRAY_BUFFER, bytes, STATIC_DRAW);
    ctx.draw_arrays(TRIANGLE_STRIP, 0, 4);
}

const NAMETABLE: [u8; 1024] = [0; 1024];

fn draw_background(
    ctx: &WebGl2RenderingContext,
    background_program: &BackgroundProgram,
    rgbav: &[f32],
) {
    ctx.use_program(Some(&background_program.shader_program));
    let buffer = ctx.create_buffer().unwrap();
    ctx.bind_buffer(ARRAY_BUFFER, Some(&buffer));
    ctx.vertex_attrib_i_pointer_with_i32(background_program.a_position, 2, UNSIGNED_INT, 16, 0);
    ctx.enable_vertex_attrib_array(background_program.a_position);
    ctx.vertex_attrib_i_pointer_with_i32(background_program.a_tex_coord, 2, UNSIGNED_INT, 16, 8);
    ctx.enable_vertex_attrib_array(background_program.a_tex_coord);
    ctx.uniform1fv_with_f32_array(Some(&background_program.u_rgbav), rgbav);
    draw_rect(&ctx, Rect::new(0, 0, 8, 8));
}

pub struct NesEmulator {
    ctx: WebGl2RenderingContext,
    background_program: BackgroundProgram,
    filename: String,
    md5: String,
    rgbav: Vec<f32>,
    console: Option<Console>,
}

fn randomize_rgbav(rgbav: &mut Vec<f32>) {
    let n = rgbav.len() / 4;
    for i in 0..n {
        rgbav[i * 4] = random::<f32>();
        rgbav[i * 4 + 1] = random::<f32>();
        rgbav[i * 4 + 2] = random::<f32>();
        rgbav[i * 4 + 3] = 1.0;
    }
}

impl NesEmulator {
    pub fn new(canvas: HtmlCanvasElement, filename: String, md5: String) -> NesEmulator {
        let (ctx, background_program) = init(&canvas).unwrap();
        let mut rgbav = vec![0.0; 256];
        randomize_rgbav(&mut rgbav);
        info!("{:?}", ctx.get_parameter(MAX_VERTEX_UNIFORM_COMPONENTS));
        info!("{:?}", ctx.get_parameter(MAX_TEXTURE_IMAGE_UNITS));
        let console = None;
        NesEmulator {
            ctx,
            background_program,
            filename,
            md5,
            rgbav,
            console,
        }
    }

    pub async fn load_rom(&mut self) -> Result<(), JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        let url = format!("/data/nes/{}", self.filename);
        let request = Request::new_with_str_and_init(&url, &opts)?;
        let bytes = fetch_bytes_with_request(&request).await?;
        let digest = md5::compute(&bytes[16..]);
        if self.md5 != format!("{:X}", digest) {
            error!("md5");
        }
        let nes_header = NesHeader::new(&bytes);
        info!("{:?}", nes_header.header_version());
        let rom = VirtualFile::new(&self.filename, &bytes);
        let console = Console::new(&rom);
        self.console = Some(console);
        Ok(())
    }

    pub fn run(mut self) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let ctx = self.ctx.clone();
        let mut last: f64 = 0.0;
        let mut queue: VecDeque<f64> = VecDeque::new();
        let mut sum: f64 = 0.0;
        let queue_size = 100;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = now();
            let delta = now - last;
            let fps = 1000.0 / delta;
            queue.push_back(fps);
            sum += fps;
            if queue.len() > queue_size {
                sum -= queue.pop_front().unwrap();
            }
            let average = sum / queue.len() as f64;
            let status = format!("FPS: {:.1}", average);
            fps_p().set_text_content(Some(&status));
            last = now;
            let gamepads = get_gamepads();
            let mut r = 0.0;
            let mut g = 0.0;
            for gamepad in gamepads {
                r = (gamepad.axes[0] + 1.0) / 2.0;
                g = (gamepad.axes[1] + 1.0) / 2.0;
            }
            randomize_rgbav(&mut self.rgbav);
            ctx.clear_color(0.0, 0.0, 0.0, 1.0);
            ctx.clear(COLOR_BUFFER_BIT);
            draw_background(&ctx, &self.background_program, &self.rgbav);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
