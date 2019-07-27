use crate::tensor::Tensor;

pub trait Module<T> {
    fn forward(&self);
    fn backward(&self);

    fn result(&self) -> &Tensor<T>;
}
