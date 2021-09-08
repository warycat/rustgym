use crate::core::{Shape, ShapeLike};
use float_cmp::ApproxEq;

pub enum TensorType {
    F32,
    I32,
    Bool,
}

pub trait TensorValue: 'static + Default + Clone + Copy + PartialEq {}

impl TensorValue for f32 {}
impl TensorValue for i32 {}
impl TensorValue for bool {}

#[derive(Debug, Clone)]
pub struct Tensor<T: TensorValue> {
    data: Vec<T>,
    shape: Vec<usize>,
}

impl<T: TensorValue> PartialEq for Tensor<T> {
    fn eq(&self, other: &Tensor<T>) -> bool {
        self.shape == other.shape && self.data == other.data
    }
}

impl<T: TensorValue> Tensor<T> {
    pub fn new(data: Vec<T>, shape: Shape) -> Box<Self> {
        assert_eq!(data.len(), shape.tensor_size());
        Box::new(Tensor { data, shape })
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn buf(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    pub fn buf_mut(&self) -> *mut T {
        self.data.as_ptr() as *mut T
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

pub trait TensorLike: std::fmt::Debug {
    fn tensor_type(&self) -> TensorType;
    fn size(&self) -> usize;
    fn shape(&self) -> &Shape;
    fn as_f32(&self) -> &Tensor<f32>;
    fn as_i32(&self) -> &Tensor<i32>;
    fn as_bool(&self) -> &Tensor<bool>;
}

impl TensorLike for Tensor<f32> {
    fn tensor_type(&self) -> TensorType {
        TensorType::F32
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn shape(&self) -> &Shape {
        self.shape()
    }
    fn as_f32(&self) -> &Tensor<f32> {
        self
    }
    fn as_i32(&self) -> &Tensor<i32> {
        panic!()
    }
    fn as_bool(&self) -> &Tensor<bool> {
        panic!()
    }
}

impl TensorLike for Tensor<i32> {
    fn tensor_type(&self) -> TensorType {
        TensorType::I32
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn shape(&self) -> &Shape {
        self.shape()
    }
    fn as_f32(&self) -> &Tensor<f32> {
        panic!()
    }
    fn as_i32(&self) -> &Tensor<i32> {
        self
    }
    fn as_bool(&self) -> &Tensor<bool> {
        panic!()
    }
}

impl TensorLike for Tensor<bool> {
    fn tensor_type(&self) -> TensorType {
        TensorType::Bool
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn shape(&self) -> &Shape {
        self.shape()
    }
    fn as_f32(&self) -> &Tensor<f32> {
        panic!()
    }
    fn as_i32(&self) -> &Tensor<i32> {
        panic!()
    }
    fn as_bool(&self) -> &Tensor<bool> {
        self
    }
}

impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M> + TensorValue> ApproxEq
    for &'a Tensor<F>
{
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.shape().eq(other.shape())
            && self.data().approx_eq(other.data(), margin)
            && self.data().approx_eq(other.data(), margin)
    }
}
