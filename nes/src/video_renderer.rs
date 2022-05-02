use crate::*;

#[derive(Default)]
pub struct VideoRenderer {
    renderer: Option<Box<dyn RenderingDevice>>,
}

impl VideoRenderer {
    pub fn new(renderer: Box<dyn RenderingDevice>) -> Self {
        let mut this = VideoRenderer::default();
        this.renderer = Some(renderer);
        this
    }

    pub fn update_frame(&self, framebuffer: &mut [u8], width: usize, height: usize) {
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.update_frame(framebuffer, width, height);
        }
    }
}
