
fn main() {
    let inputs: [f32; 4] = [1.0, 2.0, 3.0, 2.5];
    
    let weights: [[f32; 4]; 3] = [
        [0.2, 0.8, -0.5, 1.0], 
        [0.5, -0.91, 0.26, -0.5], 
        [-0.26, -0.27, 0.17, 0.87]
    ];
    
    let biases: [f32; 3] = [2.0, 3.0, 0.5];

    let mut layer_output: Vec<f32> = [].to_vec();

    for i in 0..weights.len() { // Parent loop is looping on length of weights list as this will be equal to the length of output list
        let mut neuron_val: f32 = 0.0;
        for j in 0..inputs.len() { // Child loop is looping on the inputs list as this will be equal to the length of each weight list
            neuron_val += weights[i][j] * inputs[j]
        }

        neuron_val += biases[i];
        layer_output.push(neuron_val);
    }
    println!("{:?}", layer_output)
}
