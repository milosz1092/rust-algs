use std::time::Instant;

use rust_algs::gcd::{gcd_optimal, gcd_suboptimal};

fn main() {
    let a = 1;
    let b = 999999999;

    println!("Calculate Greatest Common Divisor");
    println!("A={} B={}\n", a, b);

    let mut now = Instant::now();
    gcd_suboptimal(a, b);
    println!(
        "Suboptimal algorithm: {}ns = {:.8}s",
        now.elapsed().as_nanos(),
        now.elapsed().as_secs_f32()
    );

    now = Instant::now();
    gcd_optimal(a, b);
    println!(
        "Optimal algorithm: {}ns = {:.8}s",
        now.elapsed().as_nanos(),
        now.elapsed().as_secs_f32()
    );
}
