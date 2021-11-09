// use crate::*;
// use std::fmt;

// pub trait Layer {
//     fn name(&self) -> &str;
//     fn init_with_device(&mut self, device: id);
// }

// pub struct Dense {
//     name: String,
// }

// impl Layer for Dense {
//     fn name(&self) -> &str {
//         self.name.as_ref()
//     }

//     fn init_with_device(&mut self, device: id) {
//         println!("{:?}", device);
//     }
// }

// impl Drop for Dense {
//     fn drop(&mut self) {}
// }

// pub struct Input {
//     name: String,
//     batch: usize,
//     width: usize,
//     height: usize,
// }

// impl Input {
//     pub fn new(name: &str, batch: usize, width: usize, height: usize) -> Box<Self> {
//         Box::new(Input {
//             name: name.to_string(),
//             batch,
//             width,
//             height,
//         })
//     }
// }

// impl Layer for Input {
//     fn name(&self) -> &str {
//         self.name.as_ref()
//     }
//     fn init_with_device(&mut self, device: id) {
//         unsafe {
//             // self.image_batch = input_init(device, self.batch, self.width, self.height);
//         }
//     }
// }

// impl fmt::Debug for dyn Layer {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.name())
//     }
// }

// impl Drop for Input {
//     fn drop(&mut self) {
//         unsafe {
//             // input_release(self.image_batch);
//         }
//     }
// }
