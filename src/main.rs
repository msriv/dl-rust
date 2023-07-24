
fn main() {
    let inputs: [f32; 4] = [1.0, 2.0, 3.0, 2.5];
    let weights: [f32; 4] = [0.2, 0.8, -0.5, 1.0];
    let bias: i8 = 2;

    let output = inputs[0] as f32 * weights[0] + inputs[1] as f32 * weights[1] + inputs[2] as f32 * weights[2] + inputs[3] as f32 * weights[3] + bias as f32;
    println!("{:?}", output)
}
