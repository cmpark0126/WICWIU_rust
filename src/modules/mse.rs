use std::fmt::{Display, Debug};
use crate::numeric::Numeric;
use crate::modules::Module;
use crate::tensor::Tensor;

#[derive(Debug)]
pub struct MSE<T>{
    result: Tensor<T>
}

impl<T> MSE<T>
where T: Numeric + Clone + Display + Debug
{
    pub fn new(input: &Module<T>) -> MSE<T>{
        MSE{result: input.result().clone()}
    }
}

impl<T> Module<T> for MSE<T>
where T: Numeric + Clone + Display + Debug
{
    fn forward(&self){
        println!("forward for MSE");
    }

    fn backward(&self){
        println!("backward for MSE");
    }

    fn result(&self) -> &Tensor<T>{
        &self.result
    }
}
