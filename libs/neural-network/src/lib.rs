use std::process::Output;
use rand::Rng;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn new(&self, layers: &Vec<LayerTopology>) -> Self {
        assert!(layers.len() > 1);
        let mut build_layer = Vec::new();
        for i in 0..layers.len()-1 {
            let current = layers[i].neurons;
            let next = layers[i+1].neurons;
            build_layer.push(Layer::new(current, next))
        }
        Self {layers: build_layer}
    }
    pub fn propogate(&self, inputs:Vec<f64>) -> Vec<f64> {
        let mut signal = inputs;
        for layer in &self.layers {
            signal = layer.propagate(signal);
        }
        signal
    }

}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    fn new(input_size: usize, output_size:usize) -> Self {
        let neurons = (0..output_size).map(|_| Neuron::new(input_size)).collect();
        Self {neurons}
    }

    fn propagate(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut outputs = Vec::<f64>::New();
        for neuron in self.neurons {
            let output = neuron.propogate(&inputs);
            outputs.push(output);
        }
        outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f64,
    weights: Vec<f64>
}

impl Neuron {
    fn new(input_size: usize) -> Self{
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| todo!()).collect();
        Self {bias, weights}

    }
    fn propogate(&self, inputs:&Vec<f64>) -> f64 {
        assert_eq!(self.weights.len(), inputs.len());
        let mut output = 0.0;
        for (&input,&weight) in inputs.iter().zip(&self.weights) {
            output += input * weight;
        }

        output += self.bias;

        output.max(0.0)

    }
}