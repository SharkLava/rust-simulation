use std::process::Output;

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propogate(&self, inputs: &[f32]) -> Result<f32, String> {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "testing equality between input length {} and weight length {}",
            inputs.len(),
            self.weights.len()
        );

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        Ok((self.bias + output).max(0.0))
    }

    pub fn random(input_size: usize) -> Self {
        let bias = todo!();
        let weights = (0..input_size).map(|_| todo!()).collect();
        Self { bias, weights }
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| (neuron.propogate(&inputs)).unwrap())
            .collect()
    }

    pub fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();
        Self { neurons }
    }
}

pub struct LayerTopology {
    pub neurons: usize,
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(
            layers.len() > 1,
            "Checking if number of layers {} is greater than 1",
            layers.len()
        );
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}
