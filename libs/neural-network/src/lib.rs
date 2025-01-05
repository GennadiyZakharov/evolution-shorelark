use rand::{Rng, RngCore};
#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}



impl NeuralNetwork {
    pub fn init(layers: Vec<LayerTopology>) -> Self {
        assert!(layers.len() > 1);
        let mut built_layers: Vec<Layer> = Vec::new();
        for i in 0..(layers.len() - 1) {
            let input_size = layers[i].neurons;
            let output_size = layers[i + 1].neurons;
            built_layers.push(Layer::init(input_size, output_size));
        }
        Self {layers: built_layers}

    }
    /* here we're adding mut to binding: inputs
    it means that the can reassign inputs to the new Vector inside function
    But we can't change the content of the original reference, ig call any mutable methods

    Short:
    * mut input: Vec<f32> - we can assign input to different value, but can't chant the Vector
    input: &mut Vec<f32> - cannot reassign input vaeriable, but can change vector contect (push or pop)

    */
    pub fn propogate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propogate(&inputs);
        }
        inputs
    }
}

impl Layer {
    fn init(input_size: usize, output_size: usize) -> Self {
        let mut neurons: Vec<Neuron> = Vec::new();
        for _ in 0..output_size {
            neurons.push(Neuron::random(input_size));
        }
        Self { neurons }
    }
    fn propogate(&self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut outputs: Vec<f32> = Vec::new();
        for neuron in &self.neurons {
            let output = neuron.propogate(&inputs);
            outputs.push(output);
        }
        outputs
    }
}

impl Neuron {
    fn random(input_size: usize) -> Self {
        let mut rng = rand::rng();
        let bias = rng.random_range(-1.0..=1.0);
        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect::<Vec<f32>>();
        Self { bias, weights }
    }

    fn propogate(&self, inputs: &Vec<f32>) -> f32 {
        assert_eq!(inputs.len(),self.weights.len() );
        let mut output = 0.0;
        for (&weight, &input) in self.weights.iter().zip(inputs) {
            output += weight * input;
        }

        output += self.bias;
        output.max(0.0)
    }



}

