use rand_distr::{Distribution, Normal};
use std::fs::File;
use std::io::Write;
use std::ops::Div;

fn ou_euler_maruyama(
    theta: f64,
    mu: f64,
    sigma: f64,
    x0: f64,
    tmax: f64,
    n: u32,
    r: u32,
    seed: f64,
) -> Vec<f64> {
    let dt = tmax / (n as f64);
    let dt_micro = dt / (r as f64);

    let mut x_vec: Vec<f64> = Vec::new();
    x_vec.resize(n as usize, 0.0);
    x_vec[0] = x0;

    // Obtain dW
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    for t in 1..n {
        let t = t as usize;
        let mut dW: f64 = 0.0;
        for _ in 0..r {
            dW = dW + normal.sample(&mut rng);
        }
        dW = dW * dt_micro.sqrt();

        let dx = theta * (mu - x_vec[t - 1]) * dt + sigma * dW;
        x_vec[t] = x_vec[t - 1] + dx;
    }

    return x_vec;
}

fn main() -> std::io::Result<()> {
    let t_max = 10.0;
    let n = 1000;
    let x0 = 0.0;

    let x_vec = ou_euler_maruyama(1.0, -1.0, 0.1, x0, t_max, n, 10, 1.0);
    let mut f = File::create("ou_euler_maruyama.txt")?;

    for (pos, x) in x_vec.iter().enumerate() {
        let dt = t_max.div(n as f64);
        let t = dt * pos as f64;
        writeln!(&mut f, "{t:.2} {x:.3}")?;
    }
    f.flush()?;
    Ok(())
}
