use std::error::Error;
use std::fs::File;
use std::io::Write;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Location {
    Inside,
    Outside,
}

#[derive(Debug, PartialEq)]
struct PointLocation {
    location: Location,
    distance: f64,
}

struct Circle {
    center: Point,
    radius: f64,
}

struct Point {
    x: f64,
    y: f64,
}

fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn locate_n_point(c: &Circle, pt_list: &[Point]) -> Vec<PointLocation> {
    let mut result = Vec::new();

    for point in pt_list {
        let distance = distance_between_points(&point, &c.center);
        let location = if distance < c.radius {
            Location::Inside
        } else {
            Location::Outside
        };

        result.push(PointLocation { location, distance });
    }

    result
}

fn generate_svg(locations: &[PointLocation]) -> String {
    let mut svg = String::from("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#EEEEEE\" />\n");

    for location in locations {
        let color = match location.location {
            Location::Inside => "green",
            Location::Outside => "red",
        };

        let radius = 2.0;
        let x = location.distance + 250.0; // 将x坐标平移到SVG画布中心
        let y = 250.0 - location.distance; // 将y坐标平移到SVG画布中心

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
            x, y, radius, color
        ));
    }

    svg.push_str("</svg>");
    svg
}
fn generate_random_points(x_min: f64, x_max: f64, y_min: f64, y_max: f64, num_points: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();

    for _ in 0..num_points {
        let x = rng.gen_range(x_min..x_max);
        let y = rng.gen_range(y_min..y_max);
        points.push(Point { x, y });
    }

    points
}

fn locate_points_in_circle(circle: &Circle, points: &[Point]) -> Vec<PointLocation> {
    points.iter().map(|point| {
            let distance = distance_between_points(point, &circle.center);
            let location = if distance < circle.radius {
                Location::Inside
            } else {
                Location::Outside
            };
            PointLocation { location, distance }
        })
        .collect()
}
fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String>= std::env::args().collect();
    if args.len() != 10{
        eprintln!("Invalid");
        std::process::exit(1);
    }
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 10 {
        eprintln!("Invalid");
        std::process::exit(1);
    }

    let center_x = args[1].parse::<f64>()?;
    let center_y = args[2].parse::<f64>()?;
    let radius = args[3].parse::<f64>()?;
    let x_min = args[4].parse::<f64>()?;
    let x_max = args[5].parse::<f64>()?;
    let y_min = args[6].parse::<f64>()?;
    let y_max = args[7].parse::<f64>()?;
    let num_points = args[8].parse::<usize>()?;
    let output_name = &args[9];
    let mut rng = rand::thread_rng();
    let pt_list = generate_random_points(x_min, x_max, y_min, y_max, num_points);

    let circle = Circle {
        center: Point { x: center_x, y: center_y },
        radius,
    };


    let point_locations = locate_points_in_circle(&circle, &pt_list);

    let svg_content = generate_svg(&point_locations);

    let mut output_file = File::create(output_name)?;
    output_file.write_all(svg_content.as_bytes())?;

    Ok(())
}


