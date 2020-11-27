use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use warp::{http::Response, Filter};
mod terrain;

struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    const fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
}

const CV: [RGB; 5] = [
    RGB::new(144, 238, 144),
    RGB::new(60, 179, 113),
    RGB::new(34, 139, 34),
    RGB::new(85, 107, 47),
    RGB::new(0, 100, 0),
];

fn mountain_svg(mountain: &[terrain::Point], x: i32, y: i32, color: &RGB) -> Path {
    let mut data = Data::new().move_to((0, y));
    for p in mountain {
        data = data.line_to((p.x, p.y));
    }
    data = data.line_to((x, y)).close();

    let path = Path::new()
        .set(
            "fill",
            format!("rgb({}, {}, {})", color.r, color.g, color.b),
        )
        .set("d", data);
    path
}

#[tokio::main]
async fn main() {
    let mountain = warp::path!(i32 / i32).map(|x, y| {
        let mut doc = Document::new().set("viewBox", (0, 0, x, y));
        for c in &CV {
            let mountain = terrain::mountain(x, y, 1.2, 6);
            let m_svg = mountain_svg(&mountain, x, y, c);
            doc = doc.add(m_svg);
        }
        Response::builder()
            .header("Content-Type", "image/svg+xml")
            .body(doc.to_string())
    });

    let routes = warp::get().and(mountain);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
