use rand::Rng;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn start_points(width: i32, height: i32, margin: i32) -> (Point, Point) {
    let mut rng = rand::thread_rng();
    let top_margin = (height as f32 / (margin as f32 + 0.9)) as i32;
    let bottom_margin = (height as f32 / (margin as f32 + 0.1)) as i32;
    let x = Point {
        x: 0,
        y: rng.gen_range(top_margin, bottom_margin),
    };
    let y = Point {
        x: width,
        y: rng.gen_range(top_margin, bottom_margin),
    };
    (x, y)
}

fn midpoint(start: &Point, end: &Point, vdd: &f32) -> Point {
    let mut rng = rand::thread_rng();
    let hd = (start.x + end.x) / 2;
    let vd = (start.y + end.y) / 2;
    Point {
        x: hd,
        y: vd + (rng.gen_range(-vdd, vdd)) as i32,
    }
}

pub fn mountain(width: i32, height: i32, roughness: f32, num_iters: i32) -> Vec<Point> {
    // let size = num_iters.pow(2) + 1;
    let (start, end) = start_points(width, height, 2);
    let mut vd = ((start.y + end.y) / 2) as f32;
    let mpoint = midpoint(&start, &end, &vd);
    let mut points: Vec<Point> = vec![start, mpoint, end];
    let mut iteration = 1;
    while iteration <= num_iters {
        let curr_points = points.clone();
        let mpoints: Vec<_> = curr_points
            .iter()
            .zip(curr_points.iter().skip(1))
            .map(|(p1, p2)| midpoint(&p1, &p2, &vd))
            .collect();
        points.extend(mpoints);
        points.sort_by(|a, b| a.x.cmp(&b.x));
        vd = vd * 2f32.powf(-roughness);
        iteration = iteration + 1;
    }
    points
}
