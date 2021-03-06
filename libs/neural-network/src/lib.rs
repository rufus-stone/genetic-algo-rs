mod layer;
mod neuron;
pub mod topology;

use layer::Layer;
use topology::LayerTopology;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    /// Generate a new Network with the specified layers
    pub fn new(layers: Vec<Layer>) -> Network {
        assert!(!layers.is_empty());

        Self { layers }
    }

    /// Generate a new Network with randomly selected values for the layers/neurons/etc.
    pub fn random(prng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Network {
        let built_layers = layers
            .windows(2)
            .map(|layers| Layer::random(prng, layers[0].neurons, layers[1].neurons))
            .collect::<Vec<Layer>>();

        Network {
            layers: built_layers,
        }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // Could inputs be &[f32] instead of Vec<f32> ???
        // For each layer in self.layers, set inputs to the result of calling layer.propogate(inputs)
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use neuron::Neuron;

    #[test]
    fn random_network_creation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let layer1 = LayerTopology { neurons: 3 };
        let layer2 = LayerTopology { neurons: 2 };
        let layer3 = LayerTopology { neurons: 1 };

        // Roll a new Network with randomly chosen Neuron values in each layer
        let network = Network::random(&mut prng, &[layer1, layer2, layer3]);

        // There should 2 two layers in this network
        assert_eq!(network.layers.len(), 2);

        // The first layer should have 2 neurons
        assert_eq!(network.layers[0].neurons.len(), 2);

        // The second layer should have 1 neuron
        assert_eq!(network.layers[1].neurons.len(), 1);

        // Check the bias of the first neuron of the first layer
        approx::assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

        // Check the number of weights for the first neuron of the first layer (this is the same as the number of inputs it takes)
        assert_eq!(network.layers[0].neurons[0].weights.len(), 3);

        // Check the weights of the first neuron of the first layer
        let expected_weights = vec![0.67383957, 0.8181262, 0.26284897];
        approx::assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            expected_weights.as_slice()
        );

        // Check the bias of the second neuron of the first layer
        approx::assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

        // Check the number of weights for the second neuron of the first layer (this is the same as the number of inputs it takes)
        assert_eq!(network.layers[0].neurons[1].weights.len(), 3);

        // Check the weights of the first neuron of the first layer
        let expected_weights = vec![-0.53516835, 0.069369674, -0.7648182];
        approx::assert_relative_eq!(
            network.layers[0].neurons[1].weights.as_slice(),
            expected_weights.as_slice()
        );

        // Check the bias of the only neuron of the second layer
        approx::assert_relative_eq!(network.layers[1].neurons[0].bias, -0.102499366);

        // Check the number of weights for the only neuron of the second layer (this is the same as the number of inputs it takes)
        assert_eq!(network.layers[1].neurons[0].weights.len(), 2);

        // Check the weights of the only neuron of the second layer
        let expected_weights = vec![-0.48879617, -0.19277132];
        approx::assert_relative_eq!(
            network.layers[1].neurons[0].weights.as_slice(),
            expected_weights.as_slice()
        );
    }

    #[test]
    fn network_propogation() {
        // Build some layers
        let layer1 = Layer::new(vec![
            Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
            Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
        ]);
        let layer2 = Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]);
        let layers = vec![layer1, layer2];

        // Create a new Network with the specified layers (we'll clone them just so we can re-use the original layers vec later when checking the results)
        let network = Network::new(layers.clone());

        // Make up some fake input data
        let inputs = vec![0.5, 0.6, 0.7];

        // Calculate the propogated value given the fake input data
        let propogated = network.propagate(inputs.clone()); // We're only cloning inputs so we can re-use it when generating the expected value

        // This is effectively what the .propogate() function should be doing - propogate the first layer, pass the results into the second layer, and propgate that
        let expected = layers[1].propagate(layers[0].propagate(inputs));

        // Check the results of the propogation match what we expected
        approx::assert_relative_eq!(propogated.as_slice(), expected.as_slice());
    }
}
