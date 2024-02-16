use rand::{Rng, RngCore};

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
    fn new(_neurons: Vec<Neuron>) -> Self {
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
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ],
            );

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);

            approx::assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

            approx::assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897].as_slice()
            );

            approx::assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

            approx::assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                &[-0.5351684, 0.069369555, -0.7648182].as_slice()
            );

            assert_eq!(network.layers[1].neurons.len(), 1);

            approx::assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.48879623, -0.19277143].as_slice()
            );
        }
    }

    mod propogate {
        use super::*;
        use approx::assert_relative_eq;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };
            assert_relative_eq!(neuron.propogate(&[-10.0, -10.0]).unwrap(), 0.0,);
            assert_relative_eq!(
                neuron.propogate(&[0.5, 1.0]).unwrap(),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }
    }
}
