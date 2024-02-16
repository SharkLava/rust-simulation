struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propogate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propogate(inputs);
        }
        inputs
    }
}
