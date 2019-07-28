use crate::modules::Module;
use crate::numeric::Numeric;
use crate::tensor::Tensor;
use std::fmt::{Debug, Display};

// #[derive(Debug)]
pub struct Linear<T> {
    inputs: Vec<Box<dyn Module<T>>>,
    result: Tensor<T>,
    weight: Tensor<T>,
    bias: Tensor<T>,
    in_features: usize,
    out_features: usize,
    need_to_forward: bool,
    need_to_backward: bool,
    // forward_ptrs: Vec<&'a Box<dyn Module<T>>>,
    // backward_ptrs: Vec<&'a Box<dyn Module<T>>>,
}

impl<T> Linear<T>
where
    T: Numeric + Clone + Display + Debug + Sized,
{
    pub fn new(input: Box<dyn Module<T>>, in_features: usize, out_features: usize) -> Linear<T> {
        let weight = Tensor::<T>::zeros(vec![out_features, in_features]);
        let bias = Tensor::<T>::zeros(vec![out_features]);
        let result = Tensor::<T>::zeros(vec![out_features]);
        Linear {
            inputs: vec![input], // forward_list 추가할 때 need_to_forward 확인
            result: result,
            weight: weight,
            bias: bias,
            in_features: in_features,
            out_features: out_features,
            need_to_forward: true,
            need_to_backward: false,
            // backward_ptrs: vec![],
        }
    }
}

impl<T> Module<T> for Linear<T>
where
    T: Numeric + Clone + Display + Debug,
{
    fn forward(&mut self) {
        self.forward_prev_node();

        println!("forward for Linear");
    }

    fn forward_prev_node(&mut self){
        let input = &mut self.inputs[0];
        if (input.is_tensorholder() == false) &&
            (input.need_to_forward() == &true){
                input.forward();
                // self.inputs.pop()
                // self.backward_list.push(input)
        }
    }

    fn backward(&mut self) {
        println!("backward for Linear");
    }

    fn result(&self) -> &Tensor<T> {
        &self.result
    }

    fn result_mut(&mut self) -> &mut Tensor<T> {
        &mut self.result
    }

    fn need_to_forward(&self) -> &bool{
        &self.need_to_forward
    }
    fn need_to_forward_mut(&mut self) -> &mut bool{
        &mut self.need_to_forward
    }

    fn need_to_backward(&self) -> &bool{
        &self.need_to_backward
    }
    fn need_to_backward_mut(&mut self) -> &mut bool{
        &mut self.need_to_forward
    }
}
