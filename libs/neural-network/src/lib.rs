use rand::Rng;
use rand::RngCore;
use rand_chacha::ChaCha8Rng;

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn new() -> Self {
        todo!()
    }

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

    pub fn random(input_size: usize, rng: &mut dyn rand::RngCore) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn new(neurons: Vec<Neuron>) -> Self {
        todo!()
    }

    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| (neuron.propogate(&inputs)).unwrap())
            .collect()
    }

    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size, rng))
            .collect();

        Self::new(neurons)
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

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(
            layers.len() > 1,
            "Checking if number of layers {} is greater than 1",
            layers.len()
        );
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod random {
        use super::*;
        use rand::{random, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(4, &mut rng);

            assert_eq!(neuron.bias, -0.6255188);
            assert_eq!(
                neuron.weights,
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807,]
            );
        }
    }
}
