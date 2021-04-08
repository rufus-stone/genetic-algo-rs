use rand::Rng;
#[derive(Debug)]
pub struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // There should always be an equal number of inputs and weights (as the weights modify each input)
        assert_eq!(inputs.len(), self.weights.len());

        //let mut output = 0.0;

        // This can be re-written using .zip()
        //for i in 0..inputs.len() {
        //    output += inputs[i] * self.weights[i];
        //}

        // This can also be re-written using a .map()
        //for (&input, &weight) in inputs.iter().zip(&self.weights) {
        //    output += input * weight;
        //}

        // Take each input, multiply it by the corresponding weight, and sum all the results together
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        //output += self.bias;
        //output.max(0.0) // Return the whatever is the largest of output and 0.0

        // Finally, add the bias to the sum, and return whichever is the bigger of that value or 0.0
        (self.bias + output).max(0.0)
    }

    pub fn random(output_size: usize) -> Neuron {
        // Create a PRNG
        let mut prng = rand::thread_rng();

        // Generate a random bias (between -1 and 1 inclusive)
        let bias = prng.gen_range(-1.0..=1.0);

        // Generate random weights (between -1 and 1 inclusive)
        let weights = (0..output_size)
            .map(|_| prng.gen_range(-1.0..=1.0))
            .collect();

        Neuron { bias, weights }
    }
}
