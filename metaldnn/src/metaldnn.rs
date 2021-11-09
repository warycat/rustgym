use metaldnn_sys::*;
use std::ffi::c_void;

#[cfg(target_os = "macos")]
const LIB_SIZE: usize = 7158;

#[cfg(target_os = "ios")]
const LIB_SIZE: usize = 7142;

const LIB_PTR: &[u8; LIB_SIZE] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));

extern "C" {
    fn device_init() -> id;
    fn layer_init(view: id, device: id) -> id;
    fn library_init(device: id, buffer: *const c_void, size: usize) -> id;
    fn vertex_buffer_init(device: id) -> id;
    fn viewport_buffer_init(device: id) -> id;
    fn texture_init(device: id, width: usize, height: usize, grayscale: bool) -> id;
    fn texture_update(
        texture: id,
        width: usize,
        height: usize,
        buffer: *const c_void,
        pixel_size: usize,
    );
    fn pipeline_init(device: id, library: id) -> id;
    fn command_queue_init(device: id) -> id;
    fn redraw(
        layer: id,
        pipeline: id,
        command_queue: id,
        vertex_buffer: id,
        viewport_buffer: id,
        texture: id,
    );
}

#[no_mangle]
pub fn metaldnn_device_init() -> id {
    unsafe { device_init() }
}

#[no_mangle]
pub fn metaldnn_layer_init(view: id, device: id) -> id {
    unsafe { layer_init(view, device) }
}

#[no_mangle]
pub fn metaldnn_library_init(device: id) -> id {
    unsafe { library_init(device, LIB_PTR.as_ptr() as *const c_void, LIB_SIZE) }
}

#[no_mangle]
pub fn metaldnn_vertex_buffer_init(device: id) -> id {
    unsafe { vertex_buffer_init(device) }
}

#[no_mangle]
pub fn metaldnn_viewport_buffer_init(device: id) -> id {
    unsafe { viewport_buffer_init(device) }
}

#[no_mangle]
pub fn metaldnn_texture_init(device: id, width: usize, height: usize, grayscale: bool) -> id {
    unsafe { texture_init(device, width, height, grayscale) }
}

#[no_mangle]
pub fn metaldnn_texture_update(
    texture: id,
    width: usize,
    height: usize,
    buffer: *const c_void,
    pixel_size: usize,
) {
    unsafe { texture_update(texture, width, height, buffer, pixel_size) }
}

#[no_mangle]
pub fn metaldnn_pipeline_init(device: id, library: id) -> id {
    unsafe { pipeline_init(device, library) }
}

#[no_mangle]
pub fn metaldnn_command_queue_init(device: id) -> id {
    unsafe { command_queue_init(device) }
}

#[no_mangle]
pub fn metaldnn_redraw(
    layer: id,
    pipeline: id,
    command_queue: id,
    vertex_buffer: id,
    viewport_buffer: id,
    texture: id,
) {
    unsafe {
        redraw(
            layer,
            pipeline,
            command_queue,
            vertex_buffer,
            viewport_buffer,
            texture,
        )
    }
}

pub fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr, UTF8String];
        bytes as *const u8
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

pub fn nsstring_from_str(string: &str) -> NSString {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        NSString(obj)
    }
}
