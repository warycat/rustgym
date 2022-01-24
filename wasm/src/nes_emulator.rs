use crate::utils::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::console_log;

use web_sys::{
    HtmlCanvasElement, Request, RequestInit, RequestMode, Response, WebGl2RenderingContext,
    WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

const VERTEX_SHADER: u32 = WebGlRenderingContext::VERTEX_SHADER;
const FRAGMENT_SHADER: u32 = WebGlRenderingContext::FRAGMENT_SHADER;
const ARRAY_BUFFER: u32 = WebGlRenderingContext::ARRAY_BUFFER;
const STATIC_DRAW: u32 = WebGlRenderingContext::STATIC_DRAW;
const UNSIGNED_BYTE: u32 = WebGlRenderingContext::UNSIGNED_BYTE;
const COLOR_BUFFER_BIT: u32 = WebGlRenderingContext::COLOR_BUFFER_BIT;
const LINK_STATUS: u32 = WebGlRenderingContext::LINK_STATUS;
const COMPILE_STATUS: u32 = WebGlRenderingContext::COMPILE_STATUS;
const TRIANGLE_STRIP: u32 = WebGlRenderingContext::TRIANGLE_STRIP;

pub struct NesEmulator {
    ctx: WebGl2RenderingContext,
    uniforms: Vec<WebGlUniformLocation>,
    canvas: HtmlCanvasElement,
    filename: String,
    md5: String,
}

impl NesEmulator {
    pub fn new(canvas: HtmlCanvasElement, filename: String, md5: String) -> NesEmulator {
        // ctx.set_fill_style(&JsValue::from_str("blue"));
        // ctx.fill_rect(10., 10., 100., 100.);
        let (ctx, uniforms) = init(&canvas).unwrap();
        NesEmulator {
            ctx,
            uniforms,
            canvas,
            filename,
            md5,
        }
    }

    pub async fn load_rom(&mut self) -> Result<(), JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        let url = format!("/data/nes/{}", self.filename);
        let request = Request::new_with_str_and_init(&url, &opts)?;
        let bytes = fetch_bytes_with_request(&request).await?;
        console_dbg!("{}", bytes.len());
        Ok(())
    }

    pub fn run(self) {
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
            let gamepads = get_gamepads().unwrap();
            let mut r = 0.0;
            let mut g = 0.0;
            for gamepad in gamepads {
                let axes = gamepad.axes().to_vec();
                let axes: Vec<f64> = axes.into_iter().map(|v| v.as_f64().unwrap()).collect();
                r = (axes[0] + 1.0) / 2.0;
                g = (axes[1] + 1.0) / 2.0;
            }
            ctx.uniform4f(Some(&self.uniforms[0]), r as f32, g as f32, 0.0, 1.0);
            draw_rect(&ctx, Rect::new(100, 100, 100, 100));
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

#[derive(new)]
struct Rect {
    x: u8,
    y: u8,
    width: u8,
    height: u8,
}

fn init(
    canvas: &HtmlCanvasElement,
) -> Result<(WebGl2RenderingContext, Vec<WebGlUniformLocation>), JsValue> {
    let ctx: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(&ctx, VERTEX_SHADER, include_str!("../shaders/nes.vert"))?;
    let frag_shader = compile_shader(&ctx, FRAGMENT_SHADER, include_str!("../shaders/nes.frag"))?;
    let program = link_program(&ctx, &vert_shader, &frag_shader)?;
    ctx.use_program(Some(&program));

    let a_position = ctx.get_attrib_location(&program, "a_position") as u32;
    let u_resolution = ctx.get_uniform_location(&program, "u_resolution").unwrap();
    let u_color = ctx.get_uniform_location(&program, "u_color").unwrap();
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;

    let position_buffer = ctx.create_buffer().unwrap();
    ctx.bind_buffer(ARRAY_BUFFER, Some(&position_buffer));
    ctx.vertex_attrib_pointer_with_i32(0, 2, UNSIGNED_BYTE, false, 0, 0);
    ctx.enable_vertex_attrib_array(a_position);

    ctx.clear_color(0.0, 0.0, 0.0, 1.0);

    ctx.uniform2f(Some(&u_resolution), width, height);
    let uniforms = vec![u_color];
    Ok((ctx, uniforms))
}

fn draw_rect(ctx: &WebGl2RenderingContext, rect: Rect) {
    let positions = [
        rect.x,
        rect.y,
        rect.x + rect.width,
        rect.y,
        rect.x,
        rect.y + rect.height,
        rect.x + rect.width,
        rect.y + rect.height,
    ];
    ctx.clear(COLOR_BUFFER_BIT);
    ctx.buffer_data_with_u8_array(ARRAY_BUFFER, &positions, STATIC_DRAW);
    ctx.draw_arrays(TRIANGLE_STRIP, 0, 4);
}

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
