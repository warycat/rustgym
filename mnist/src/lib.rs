#[macro_use]
use objc::*;

use gilrs::ev::filter::{Filter, Repeat};
use gilrs::GilrsBuilder;
use metaldnn::*;
use rustgym_mnist_data::*;
use std::ffi::c_void;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, TouchPhase, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

#[cfg(target_os = "ios")]
use winit::platform::ios::WindowBuilderExtIOS;
#[cfg(target_os = "ios")]
use winit::platform::ios::WindowExtIOS;

#[cfg(target_os = "macos")]
fn get_view(window: &Window) -> id {
    window.ns_view() as id
}

#[cfg(target_os = "ios")]
fn get_view(window: &Window) -> id {
    window.ui_view() as id
}

struct GameState {
    pub index: usize,
}

#[no_mangle]
pub fn mainloop() {
    let mnist_data = load_mnist_data().unwrap();
    let MnistData {
        train_images,
        test_images,
        train_labels,
        test_labels,
    } = mnist_data;

    let mut gilrs = match GilrsBuilder::new().set_update_state(false).build() {
        Ok(g) => g,
        Err(gilrs::Error::NotImplemented(g)) => {
            eprintln!("Current platform is not supported");

            g
        }
        Err(e) => {
            eprintln!("Failed to create gilrs context: {}", e);
            std::process::exit(-1);
        }
    };

    let mut game_state = GameState { index: 0 };

    let repeat_filter = Repeat::new();

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop, 1024, 768);
    let device = metaldnn_device_init() as id;
    let layer = metaldnn_layer_init(get_view(&window), device);
    let library = metaldnn_library_init(device);
    let vertex_buffer = metaldnn_vertex_buffer_init(device);
    let viewport_buffer = metaldnn_viewport_buffer_init(device);
    let texture = metaldnn_texture_init(device, 28, 28, true);
    metaldnn_texture_update(
        texture,
        28,
        28,
        train_images[game_state.index].as_ptr() as *const c_void,
        1,
    );
    let pipeline = metaldnn_pipeline_init(device, library);
    let command_queue = metaldnn_command_queue_init(device);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                handle_window_event(event, control_flow, &mut game_state);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                while let Some(ev) = gilrs.next_event().filter_ev(&repeat_filter, &mut gilrs) {
                    let gilrs::Event { event, .. } = ev;
                    gilrs.update(&ev);
                    println!("{:?}", event);
                }
                metaldnn_texture_update(
                    texture,
                    28,
                    28,
                    train_images[game_state.index].as_ptr() as *const c_void,
                    1,
                );
                metaldnn_redraw(
                    layer,
                    pipeline,
                    command_queue,
                    vertex_buffer,
                    viewport_buffer,
                    texture,
                );
            }
            _ => {}
        }
    });
}

#[cfg(target_os = "macos")]
fn init_window(event_loop: &EventLoop<()>, width: u32, height: u32) -> Window {
    let window_size = LogicalSize::new(width, height);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("Rustgym Mnist")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    return window;
}

#[cfg(target_os = "ios")]
fn init_window(event_loop: &EventLoop<()>, width: u32, height: u32) -> Window {
    let window_size = LogicalSize::new(width, height);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_root_view_class((unsafe { class!(MyView) as *const _ as *const _ }))
        .with_title("Rustgym Mnist")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    return window;
}

fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    game_state: &mut GameState,
) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::MouseInput {
            device_id,
            state,
            button,
            modifiers,
        } => {
            if state == ElementState::Pressed {
                println!("Pressed");
                game_state.index += 1;
                game_state.index %= 60000;
            }
        }
        WindowEvent::Touch(touch) => match touch.phase {
            TouchPhase::Ended => {
                println!("Touched");
                game_state.index += 1;
                game_state.index %= 60000;
            }
            _ => {}
        },
        _ => {}
    }
}
