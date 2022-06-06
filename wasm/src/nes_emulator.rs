use crate::*;
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
    WebGlProgram, WebGlTexture, WebGlUniformLocation,
};

struct Program {
    shader_program: WebGlProgram,
    a_position: u32,
}

fn init(canvas: &HtmlCanvasElement) -> Result<(WebGl2RenderingContext, Program), JsValue> {
    let mut attributes: WebGlContextAttributes = WebGlContextAttributes::default();
    attributes.antialias(false);
    let ctx: WebGl2RenderingContext = canvas
        .get_context_with_context_options("webgl2", &attributes)?
        .unwrap()
        .dyn_into()?;
    let attributes = ctx.get_context_attributes().unwrap();
    info!("{:?}", attributes);
    let nes_vert_shader = compile_shader(&ctx, VERTEX_SHADER, include_str!("../shaders/nes.vert"))?;
    let nes_frag_shader =
        compile_shader(&ctx, FRAGMENT_SHADER, include_str!("../shaders/nes.frag"))?;
    let shader_program = link_program(&ctx, &nes_vert_shader, &nes_frag_shader)?;
    let a_position = ctx.get_attrib_location(&shader_program, "a_position") as u32;

    let texture: WebGlTexture = ctx.create_texture().unwrap();
    ctx.bind_texture(TEXTURE_2D, Some(&texture));
    ctx.tex_parameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
    ctx.tex_parameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
    ctx.active_texture(TEXTURE0);

    let program = Program {
        shader_program,
        a_position,
    };

    Ok((ctx, program))
}

fn draw(ctx: &WebGl2RenderingContext, program: &Program, pixels: &[u8; 256 * 256 * 4]) {
    ctx.use_program(Some(&program.shader_program));
    let buffer = ctx.create_buffer().unwrap();
    ctx.bind_buffer(ARRAY_BUFFER, Some(&buffer));
    ctx.vertex_attrib_i_pointer_with_i32(program.a_position, 2, UNSIGNED_INT, 8, 0);
    ctx.enable_vertex_attrib_array(program.a_position);
    let positions = [0, 0, 256, 0, 0, 256, 256, 256];
    let (_, bytes, _) = unsafe { positions.align_to::<u8>() };
    ctx.buffer_data_with_u8_array(ARRAY_BUFFER, bytes, STATIC_DRAW);
    ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        TEXTURE_2D,
        0,
        RGBA as i32,
        256,
        256,
        0,
        RGBA,
        UNSIGNED_BYTE,
        Some(pixels),
    )
    .unwrap();
    ctx.draw_arrays(TRIANGLE_STRIP, 0, 4);
}

pub struct NesEmulator {
    ctx: WebGl2RenderingContext,
    program: Program,
    filename: String,
    md5: String,
    console: Option<Console>,
}

impl NesEmulator {
    pub fn new(canvas: HtmlCanvasElement, filename: String, md5: String) -> NesEmulator {
        let (ctx, program) = init(&canvas).unwrap();
        info!("{:?}", ctx.get_parameter(MAX_VERTEX_UNIFORM_COMPONENTS));
        info!("{:?}", ctx.get_parameter(MAX_TEXTURE_IMAGE_UNITS));
        let console = None;
        NesEmulator {
            ctx,
            program,
            filename,
            md5,
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
        let rom = VirtualFile::new(&self.filename, &bytes);
        let console = Console::new(&rom, Box::new(BaseRenderer::default()));
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

        let mut console = self.console.take().unwrap();
        console.control_manager = ControlManager::new(EmulationFlags::default(), || get_gamepads());
        let mut bytes = [0; 256 * 256 * 4];
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            PAUSED.with(|value| {
                let paused = *value.borrow();
                if !paused {
                    console.run_single_frame();
                    randomize_rgbav(&mut bytes);
                }
            });
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
            ctx.clear_color(0.0, 0.0, 0.0, 1.0);
            ctx.clear(COLOR_BUFFER_BIT);
            draw(&ctx, &self.program, &bytes);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn randomize_rgbav(rgbav: &mut [u8]) {
    let n = rgbav.len() / 4;
    for i in 0..n {
        rgbav[i * 4] = random::<u8>();
        rgbav[i * 4 + 1] = random::<u8>();
        rgbav[i * 4 + 2] = random::<u8>();
        rgbav[i * 4 + 3] = 255;
    }
}
