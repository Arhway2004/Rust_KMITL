use rand::Rng;
use std::{fs::File, io::Write};
use std::io::Result;
use svg::Document;
use svg::node::element::Circle;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct CirclePara {
    pub center_x: f64,
    pub center_y: f64,
    pub r: f64,
}

pub struct CirclePara2 {
    pub center_x2: f64,
    pub center_y2: f64,
    pub r2: f64,
}

pub struct Bound {
    pub circle1: CirclePara,
    pub circle2: CirclePara2,
}

pub struct RangeConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug)]
pub enum Locate {
    Inside(f64),
    Outside(f64),
}

#[derive(Debug)]
pub enum Locate2 {
    InsideBoth,
    OnFirstCircle,
    OnSecondCircle,
    OutsideBoth,
}

pub fn gen_point_list<R: Rng>(rng: &mut R, cfg: &RangeConfig, n: i64) -> Vec<Point> {
    let mut result = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_min..=cfg.x_max);
        let y = rng.gen_range(cfg.y_min..=cfg.y_max);
        result.push(Point { x, y });
    }
    result
}

pub fn locate_n_point(c: &CirclePara, pt_list: &[Point]) -> Vec<Locate> {
    let mut result = Vec::new();
    for pt in pt_list {
        let d = ((pt.x - c.center_x).powi(2) + (pt.y - c.center_y).powi(2)).sqrt();
        if d < c.r {
            result.push(Locate::Inside(d));
        } else {
            result.push(Locate::Outside(d));
        }
    }
    result
}

pub fn locate_n_point2(b: &Bound, pt_list: &[Point]) -> Vec<Locate2> {
    let mut result = Vec::new();
    for pt in pt_list {
        let d1 = ((pt.x - b.circle1.center_x).powi(2) + (pt.y - b.circle1.center_y).powi(2)).sqrt();
        let d2 = ((pt.x - b.circle2.center_x2).powi(2) + (pt.y - b.circle2.center_y2).powi(2)).sqrt();
        if d1 < b.circle1.r && d2 < b.circle2.r2 {
            result.push(Locate2::InsideBoth);
        } else if d1 < b.circle1.r {
            result.push(Locate2::OnFirstCircle);
        } else if d2 < b.circle2.r2 {
            result.push(Locate2::OnSecondCircle);
        } else {
            result.push(Locate2::OutsideBoth);
        }
    }
    result
}

fn create_svg(c: &CirclePara, points: &[Point], locations: &[Locate]) -> Document {
    let mut doc = Document::new().set("viewBox", (0, 0, 500, 500));

    // Draw the circle
    let circle_element = Circle::new()
        .set("cx", c.center_x)
        .set("cy", c.center_y)
        .set("r", c.r)
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 2);
    doc = doc.add(circle_element);

    // Draw points and color them based on their location
    for (point, location) in points.iter().zip(locations.iter()) {
        let color = match location {
            Locate::Inside(_) => "green",
            Locate::Outside(_) => "red",
        };
        let point_element = Circle::new()
            .set("cx", point.x)
            .set("cy", point.y)
            .set("r", 3.0) // Adjust the radius as needed
            .set("fill", color);
        doc = doc.add(point_element);
    }

    doc
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 10 {
        eprintln!(
            "Usage: {} <x> <y> <r> <n> <x_min> <x_max> <y_min> <y_max> <output_file>",
            args[0]
        );
        std::process::exit(1);
    }

    let center_x = args[1].parse::<f64>().expect("Invalid x coordinate");
    let center_y = args[2].parse::<f64>().expect("Invalid y coordinate");
    let r = args[3].parse::<f64>().expect("Invalid radius");
    let num_points = args[4].parse::<i64>().expect("Invalid number of points");

    let circle = CirclePara {
        center_x,
        center_y,
        r,
    };

    let range_config = RangeConfig {
        x_min: args[5].parse::<f64>().expect("Invalid x_min"),
        x_max: args[6].parse::<f64>().expect("Invalid x_max"),
        y_min: args[7].parse::<f64>().expect("Invalid y_min"),
        y_max: args[8].parse::<f64>().expect("Invalid y_max"),
    };

    let output_file = &args[9];

    let mut rng = rand::thread_rng();
    let points = gen_point_list(&mut rng, &range_config, num_points);
    let locations = locate_n_point(&circle, &points);

    let svg_doc = create_svg(&circle, &points, &locations);

    let mut file = File::create(output_file)?;
    svg::write(&mut file, &svg_doc)?;

    println!("SVG image saved as '{}'", output_file);

    Ok(())
}
