use crate::neuron::Neuron;
#[derive(Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // This can be re-written using .map()
        // Using the .iter() method also implicitely calls Vec::with_capacity() which is nice
        //let mut outputs = Vec::with_capacity(self.neurons.len());
        //for neuron in &self.neurons {
        //    let output = neuron.propagate(&inputs);
        //    outputs.push(output);
        //}
        //outputs

        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(input_neurons: usize, output_neurons: usize) -> Layer {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();

        Layer { neurons }
    }
}
