pub type Shape = Vec<usize>;

pub trait ShapeLike {
    fn tensor_size(&self) -> usize;
    fn rank(&self) -> usize;
    fn buf(&self) -> *const usize;
}

impl ShapeLike for Shape {
    fn tensor_size(&self) -> usize {
        self.iter().product()
    }
    fn rank(&self) -> usize {
        self.len()
    }
    fn buf(&self) -> *const usize {
        self.as_ptr() as *const usize
    }
}
