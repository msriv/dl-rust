
fn main() {
    let inputs: [f32; 4] = [1.0, 2.0, 3.0, 2.5];
    
    let weights1: [f32; 4] = [0.2, 0.8, -0.5, 1.0];
    let weights2: [f32; 4] = [0.5, -0.91, 0.26, -0.5];
    let weights3: [f32; 4] = [-0.26, -0.27, 0.17, 0.87];
    
    let bias1: i8 = 2;
    let bias2: i8 = 3;
    let bias3: f32 = 0.5;

    let output = [
        inputs[0] as f32 * weights1[0] + inputs[1] as f32 * weights1[1] + inputs[2] as f32 * weights1[2] + inputs[3] as f32 * weights1[3] + bias1 as f32,
        inputs[0] as f32 * weights2[0] + inputs[1] as f32 * weights2[1] + inputs[2] as f32 * weights2[2] + inputs[3] as f32 * weights2[3] + bias2 as f32,
        inputs[0] as f32 * weights3[0] + inputs[1] as f32 * weights3[1] + inputs[2] as f32 * weights3[2] + inputs[3] as f32 * weights3[3] + bias3 as f32,
    ];
    println!("{:?}", output)
}
