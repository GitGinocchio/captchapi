use usvg::fontdb;
use worker::*;

use std::{collections::HashMap};

use crate::utils::*;

use base64::{engine::general_purpose, Engine as _};
use resvg::usvg::{Tree, Options};
use resvg::render;
use tiny_skia::Pixmap;
use serde_json::json;
use getrandom::fill;

const VALID_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const SVG_TEMPLATE: &str = include_str!("../assets/template.svg");

pub async fn get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let default_format = String::from("json");
    let query: HashMap<String, String> = url.query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    // api params
    let format = ctx.param("format").unwrap_or(&default_format);
    let height: f32 = match get_param(&query, "height", ParamValue::Single(50.0)) {
        ParamValue::List(_) => 50.0,
        ParamValue::Range(_, _) => 50.0,
        ParamValue::Single(value) => value
    };
    let width: f32 = match get_param(&query, "width", ParamValue::Single(250.0)) {
        ParamValue::List(_) => 250.0,
        ParamValue::Range(_, _) => 250.0,
        ParamValue::Single(value) => value
    };
    let scale: f32 = match get_param(&query, "scale", ParamValue::Single(1.0)) {
        ParamValue::List(_) => 1.0,
        ParamValue::Range(_, _) => 1.0,
        ParamValue::Single(value) => value
    };
    let blur: ParamValue<f32> = get_param(&query, "blur", ParamValue::Range(0.0, 0.5));
    let opacity: ParamValue<f32> = get_param(&query, "opacity", ParamValue::Range(0.3, 1.0));
    let fsize: ParamValue<f32> = get_param(&query, "fsize", ParamValue::Range(18.0, 30.0));
    let rotation: ParamValue<f32> = get_param(&query, "rotation", ParamValue::Range(-15.0, 15.0));
    let nletters: u32 = match get_param(&query, "nletters", ParamValue::Range(4, 6)) {
        ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_u32(4, 6)),
        ParamValue::Range(min, max) => random_range_u32(min, max),
        ParamValue::Single(value) => value
    };
    let lscalex: ParamValue<f32> = get_param(&query, "lscalex",ParamValue::Range(0.8, 1.2));
    let lscaley: ParamValue<f32> = get_param(&query, "lscaley", ParamValue::Range(0.8, 1.2));
    let lwidth: ParamValue<f32> = get_param(&query, "lwidth", ParamValue::Range(1.0, 3.0));
    let lopacity: ParamValue<f32> = get_param(&query, "lopacity", ParamValue::Range(0.1, 0.6));
    let nlines: u32 = match get_param(&query, "nlines", ParamValue::Range(3, 5)) {
        ParamValue::List(_) => random_range_u32(3, 5),
        ParamValue::Range(min, max) => random_range_u32(min, max),
        ParamValue::Single(value) => value
    };
    let cradius: ParamValue<f32> = get_param(&query, "cradius", ParamValue::Range(1.0, 3.0));
    let copacity: ParamValue<f32> = get_param(&query, "copacity", ParamValue::Range(0.1, 0.8));
    let ncircles: u32 = match get_param(&query, "ncircles", ParamValue::Range(5, 10)) {
        ParamValue::List(_) => random_range_u32(5, 10),
        ParamValue::Range(min, max) => random_range_u32(min, max),
        ParamValue::Single(value) => value
    };
    let npaths: u32 = match get_param(&query, "npaths", ParamValue::Range(3, 5)) {
        ParamValue::List(_) => random_range_u32(3, 5),
        ParamValue::Range(min, max) => random_range_u32(min, max),
        ParamValue::Single(value) => value
    };
    let pstroke: ParamValue<f32> = get_param(&query, "pstroke", ParamValue::Range(1.0, 3.0));
    let plenght: ParamValue<f32> = get_param(&query,"plenght", ParamValue::Range(10.0, 60.0));
    let popacity: ParamValue<f32> = get_param(&query, "popacity", ParamValue::Range(0.1, 0.6));

    console_log!("Query: {query:?}");

    let letters = generate_random_letters(nletters as usize);
    let svg = generate_svg(
        &letters.as_str(), 
        width * scale, 
        height * scale, 
        blur,
        opacity,
        fsize,
        rotation,
        lscalex,
        lscaley,
        lwidth,
        lopacity,
        cradius,
        copacity,
        pstroke,
        plenght,
        popacity,
        nlines,
        ncircles,
        npaths,
    );

    match format.as_str() {
        "svg" => {
            let mut response = Response::from_body(ResponseBody::Body(svg.as_bytes().to_vec()))?;
            response.headers_mut().append("Content-Type", "image/svg+xml")?;

            Ok(response)
        }
        "png" => {
            let font_bytes: &[u8] = include_bytes!("../assets/fonts/playfair-display/PlayfairDisplay-Regular.ttf");
            
            let mut fontdb = fontdb::Database::new();
            fontdb.load_font_data(font_bytes.to_vec());
            fontdb.set_serif_family("Playfair Display");

            let mut opt = Options::default();
            opt.fontdb = fontdb.into();

            let rtree = Tree::from_str(&svg.as_str(), &opt).expect("Failed to parse SVG");
            let size = rtree.size();

            let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).expect("Failed to create Pixmap");
            render(&rtree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

            let png_bytes = pixmap.encode_png().expect("Failed to encode PNG");

            let mut response = Response::from_body(ResponseBody::Body(png_bytes))?;
            response.headers_mut().append("Content-Type", "image/png")?;

            Ok(response)
        }
        "xml" | "xml+svg" => {
            let svg_base64 = general_purpose::STANDARD.encode(svg.as_bytes());

            let payload = format!(
                r#"<response>
                    <image>data:image/svg+xml;base64,{}</image>
                    <text>{}</text>
                   </response>"#,
                svg_base64, letters
            );

            let mut response = Response::from_body(ResponseBody::Body(payload.into_bytes()))?;
            response.headers_mut().append("Content-Type", "application/xml")?;
            response.headers_mut().append("Access-Control-Allow-Origin", "*")?;
            response.headers_mut().append("Access-Control-Allow-Headers", "Content-Type")?;
            Ok(response)
        }
        "xml+png" => {
            let font_bytes: &[u8] = include_bytes!("../assets/fonts/playfair-display/PlayfairDisplay-Regular.ttf");
            
            let mut fontdb = fontdb::Database::new();
            fontdb.load_font_data(font_bytes.to_vec());
            fontdb.set_serif_family("Playfair Display");

            let mut opt = Options::default();
            opt.fontdb = fontdb.into();

            let rtree = Tree::from_str(&svg.as_str(), &opt).expect("Failed to parse SVG");
            let size = rtree.size();

            let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).expect("Failed to create Pixmap");
            render(&rtree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

            let png_bytes = pixmap.encode_png().expect("Failed to encode PNG");

            let png_base64 = general_purpose::STANDARD.encode(png_bytes);

            let payload = format!(
                r#"<response>
                    <image>data:image/svg+xml;base64,{}</image>
                    <text>{}</text>
                   </response>"#,
                png_base64, letters
            );

            let mut response = Response::from_body(ResponseBody::Body(payload.into_bytes()))?;
            response.headers_mut().append("Content-Type", "application/xml")?;
            response.headers_mut().append("Access-Control-Allow-Origin", "*")?;
            response.headers_mut().append("Access-Control-Allow-Headers", "Content-Type")?;
            Ok(response)
        }
        "json+png" => {
            let font_bytes: &[u8] = include_bytes!("../assets/fonts/playfair-display/PlayfairDisplay-Regular.ttf");
            
            let mut fontdb = fontdb::Database::new();
            fontdb.load_font_data(font_bytes.to_vec());
            fontdb.set_serif_family("Playfair Display");

            let mut opt = Options::default();
            opt.fontdb = fontdb.into();

            let rtree = Tree::from_str(&svg.as_str(), &opt).expect("Failed to parse SVG");
            let size = rtree.size();

            let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32).expect("Failed to create Pixmap");
            render(&rtree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

            let png_bytes = pixmap.encode_png().expect("Failed to encode PNG");

            let png_base64 = general_purpose::STANDARD.encode(png_bytes);

            let payload = json!({
                "text" : letters,
                "image": format!("data:image/png;base64,{}", png_base64)
            });

            let mut response = Response::from_json(&payload)?;
            response.headers_mut().append("Content-Type", "application/json")?;
            response.headers_mut().append("Access-Control-Allow-Origin", "*")?;
            response.headers_mut().append("Access-Control-Allow-Headers", "Content-Type")?;

            Ok(response)
        }
        "json" | "json+svg" | _ => {
            let svg_base64 = general_purpose::STANDARD.encode(svg.as_bytes());

            let payload = json!({
                "text" : letters,
                "image": format!("data:image/svg+xml;base64,{}", svg_base64)
            });

            let mut response = Response::from_json(&payload)?;
            response.headers_mut().append("Content-Type", "application/json")?;
            response.headers_mut().append("Access-Control-Allow-Origin", "*")?;
            response.headers_mut().append("Access-Control-Allow-Headers", "Content-Type")?;

            Ok(response)
        }
    }
}

fn generate_svg(
    word: &str,
    width: f32,
    height: f32,
    blur: ParamValue<f32>,
    opacity: ParamValue<f32>,
    fsize: ParamValue<f32>,
    rotation: ParamValue<f32>,
    lscalex: ParamValue<f32>,
    lscaley: ParamValue<f32>,
    lwidth: ParamValue<f32>,
    lopacity: ParamValue<f32>,
    cradius: ParamValue<f32>,
    copacity: ParamValue<f32>,
    pstroke: ParamValue<f32>,
    plenght: ParamValue<f32>,
    popacity: ParamValue<f32>,
    nlines: u32,
    ncircles: u32,
    npaths : u32
) -> String {
    let canvas_width = 250.0;
    let canvas_height = 50.0;
    let center_y = canvas_height / 2.0;
    let spacing = 10.0;

    let mut total_width = 0.0;
    let mut dimensions = Vec::new();
    let mut filters = String::new();

    for _ in 0..word.len() {
        let fsize = match fsize {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(18.0, 30.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };
        let lscalex = match lscalex {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.8, 1.2)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };
        let rotation = match rotation {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(-15.0, 15.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        let base_width = fsize * lscalex;
        let rotation_factor = 1.0 + (rotation.to_radians().sin().abs() * 0.5);
        let letter_width = base_width * rotation_factor + spacing;

        dimensions.push((fsize, lscalex, rotation, letter_width));
        total_width += letter_width;
    }

    let mut x_pos = (canvas_width - total_width) / 2.0 + total_width * 0.07;
    let mut svg_letters = String::new();

    for (i, char) in word.chars().enumerate() {
        let (fsize, lscalex, rotation, letter_width) = dimensions[i];
        
        let lscaley = match lscaley {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.8, 1.2)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        let blur = match blur {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.0, 0.5)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        let opacity = match opacity  {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.3, 1.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        filters.push_str(&format!(
            r#"<filter id="blur-{blur:.1}" x="-20%" y="-20%" width="140%" height="140%">
                <feGaussianBlur in="SourceGraphic" stdDeviation="{blur:.1}" />
            </filter>"#
        ));

        svg_letters.push_str(&format!(
            r#"<g 
                transform="translate({x_pos:.1}, {center_y:.1}) rotate({rotation:.1}) scale({lscalex:.2},{lscaley:.2})"
                filter="url(#blur-{blur:.1})">
                <text 
                    x="0"
                    y="0"
                    font-family="Playfair Display"
                    font-size="{fsize:.1}" 
                    fill="black"
                    fill-opacity="{opacity}"
                    dominant-baseline="middle" 
                    text-anchor="middle">
                    {char}
                </text>
               </g>"#
        ));

        x_pos += letter_width;
    }

    let noise_lines = generate_noise_lines(nlines as usize, canvas_width, canvas_height, lwidth, lopacity);
    let noise_circles = generate_noise_circles(ncircles as usize, canvas_width, canvas_height, cradius, copacity);
    let noise_paths = generate_noise_paths(npaths as usize, canvas_width, canvas_height, pstroke, plenght, popacity, canvas_height);

    SVG_TEMPLATE
        .replace("{{filters}}", &filters)
        .replace("{{height}}", &height.to_string().as_str())
        .replace("{{width}}", &width.to_string().as_str())
        .replace("{{letters}}", &svg_letters)
        .replace("{{lines}}", &noise_lines)
        .replace("{{circles}}", &noise_circles)
        .replace("{{paths}}", &noise_paths)
}

fn generate_random_letters(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    fill(&mut bytes).expect("Failed to get random bytes");

    bytes.iter()
        .map(|b| {
            let index = (*b as usize) % VALID_CHARS.len();
            VALID_CHARS[index] as char
        })
        .collect()
}

fn generate_noise_lines(count: usize, width: f32, height: f32, line_width: ParamValue<f32>, line_opacity: ParamValue<f32>) -> String {
    let mut lines = String::new();

    for _ in 0..count {
        let x1 = random_range_f32(0.0, width);
        let y1 = random_range_f32(0.0, height);
        let x2 = random_range_f32(0.0, width);
        let y2 = random_range_f32(0.0, height);

        let stroke_width = match line_width {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(1.0, 3.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };
        let line_opacity = match line_opacity {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.1, 1.0)),
            ParamValue::Range(min, max) => random_range_f32(min,max),
            ParamValue::Single(value) => value
        };

        lines.push_str(&format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="black" stroke-opacity="{:.2}" stroke-width="{:.1}" />"#,
            x1, y1, x2, y2, line_opacity, stroke_width
        ));
    }

    lines
}

fn generate_noise_paths(count: usize, width: f32, height: f32, stroke: ParamValue<f32>, length : ParamValue<f32>, opacity: ParamValue<f32>, ctrl_height_range: f32) -> String {
    let mut paths = String::new();

    for _ in 0..count {
        let x1 = random_range_f32(0.0, width);
        let y1 = random_range_f32(0.0, height);

        // Calcolo spostamento
        let angle = random_range_f32(0.0, std::f32::consts::TAU); // angolo tra 0 e 2π
        let length = match length {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(10.0, 60.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };
        let stroke = match stroke {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(1.0, 3.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        let x2 = (x1 + length * angle.cos()).clamp(0.0, width);
        let y2 = (y1 + length * angle.sin()).clamp(0.0, height);

        let ctrl_x = random_range_f32(0.0, width);
        let ctrl_y = random_range_f32(0.0, ctrl_height_range);
        let opacity = match opacity {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.3, 1.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        paths.push_str(&format!(
            r#"<path d="M {:.1} {:.1} Q {:.1} {:.1}, {:.1} {:.1}" stroke="black" stroke-opacity="{:.2}" stroke-width="{:.1}" fill="none" />"#,
            x1, y1, ctrl_x, ctrl_y, x2, y2, opacity, stroke
        ));
    }

    paths
}

fn generate_noise_circles(count: usize, width: f32, height: f32, radius: ParamValue<f32>, opacity: ParamValue<f32>) -> String {
    let mut circles = String::new();

    for _ in 0..count {
        let cx = random_range_f32(0.0, width);
        let cy = random_range_f32(0.0,height);

        let radius = match radius {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(1.0, 3.0)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };
        let opacity = match opacity {
            ParamValue::List(ref vec) => *random_element(&vec).unwrap_or(&random_range_f32(0.05, 0.25)),
            ParamValue::Range(min, max) => random_range_f32(min, max),
            ParamValue::Single(value) => value
        };

        let gray = random_range_u32(0, 255);

        circles.push_str(&format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="rgb({},{},{})" fill-opacity="{:.2}" />"#,
            cx, cy, radius, gray, gray, gray, opacity
        ));
    }

    circles
}