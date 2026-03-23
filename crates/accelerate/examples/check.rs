//! Accelerate vDSP + BLAS demo.
//!
//! cargo run -p rswift-accelerate --example check

fn main() {
    println!("=== Accelerate: vDSP + BLAS ===\n");
    println!("Available: {}", accelerate::is_available());

    let a = vec![1.0f32, 2.0, 3.0, 4.0];
    let b = vec![5.0f32, 6.0, 7.0, 8.0];

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!();

    println!("add:   {:?}", accelerate::vdsp::add_f32(&a, &b));
    println!("sub:   {:?}", accelerate::vdsp::sub_f32(&a, &b));
    println!("mul:   {:?}", accelerate::vdsp::mul_f32(&a, &b));
    println!("scale: {:?}", accelerate::vdsp::scale_f32(&a, 10.0));
    println!("dot:   {}", accelerate::vdsp::dot_f32(&a, &b));
    println!("sum:   {}", accelerate::vdsp::sum_f32(&a));
    println!("mean:  {}", accelerate::vdsp::mean_f32(&a));
    println!("max:   {}", accelerate::vdsp::max_f32(&a));
    println!("min:   {}", accelerate::vdsp::min_f32(&a));
    println!("rms:   {:.4}", accelerate::vdsp::rms_f32(&a));

    // Matrix multiply: 2×2 × 2×2
    println!("\n=== BLAS: Matrix Multiply ===\n");
    let m_a = vec![1.0f32, 2.0, 3.0, 4.0]; // [[1,2],[3,4]]
    let m_b = vec![5.0f32, 6.0, 7.0, 8.0]; // [[5,6],[7,8]]
    let c = accelerate::blas::sgemm(&m_a, &m_b, 2, 2, 2);
    println!("[1 2] × [5 6] = [{:2} {:2}]", c[0], c[1]);
    println!("[3 4]   [7 8]   [{:2} {:2}]", c[2], c[3]);
}
