use rand::Rng;
use std::env;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct RandConfig {
    x_inside: f64,
    x_outside: f64,
    y_inside: f64,
    y_outside: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

pub fn gen_point_list<R: Rng>(rng: &mut R, cfg: &RandConfig, n: usize) -> Vec<Point> {
    let mut result = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_inside..cfg.x_outside);
        let y = rng.gen_range(cfg.y_inside..cfg.y_outside);

        let point = Point { x, y };
        result.push(point);
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <num-points>", args[0]);
        std::process::exit(1);
    }

    let n_str = &args[1];
    let n = n_str.parse::<usize>()?;

    let mut rng = rand::thread_rng();
    let cfg = RandConfig {
        x_inside: -1.5,
        x_outside: 1.5,
        y_inside: -1.5,
        y_outside: 1.5,
    };

    let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, n);

    for pt in &pt_list {
        println!("{:?}", pt);
    }

    Ok(())
}
#[test]
fn test_gen_point_list() {
    let mut rng = rand::thread_rng();
    let cfg = RandConfig {
        x_inside: -1.5,
        x_outside: 1.5,
        y_inside: -1.5,
        y_outside: 1.5,
    };

    // Define your expectations here
    let n = 10; // Change this to the desired number of points
    let point_list = gen_point_list(&mut rng, &cfg, n);

    // You can check various properties of the generated point list
    assert_eq!(point_list.len(), n); // Check the number of points

    for point in &point_list {
        assert!(point.x >= cfg.x_inside && point.x <= cfg.x_outside);
        assert!(point.y >= cfg.y_inside && point.y <= cfg.y_outside);
    }
}