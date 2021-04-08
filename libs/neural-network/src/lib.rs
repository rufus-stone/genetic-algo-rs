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
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // This can be re-written using .fold()
        //for layer in &self.layers {
        //    inputs = layer.propagate(inputs);
        //}
        //inputs

        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(layers: &[LayerTopology]) -> Network {
        let built_layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Network {
            layers: built_layers,
        }
    }
}
