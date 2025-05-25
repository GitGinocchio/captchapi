use worker::*;

use base64::{engine::general_purpose, Engine as _};
use serde_json::json;
use getrandom::fill;

const VALID_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

const SVG_TEMPLATE: &str = include_str!("../static/template.svg");

pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let _query_params = url.query();

    let url = req.url()?;
    let segments: Vec<_> = url.path_segments().map_or(Vec::new(), |s| s.collect());

    let letters = generate_random_letters(6);
    let svg = generate_svg(&letters.as_str());

    match segments.as_slice() {
        ["captcha", "show"] => {
            let mut response = Response::from_body(ResponseBody::Body(svg.as_bytes().to_vec()))?;
            response.headers_mut().append("Content-Type", "image/svg+xml")?;

            Ok(response)
        }
        ["captcha"] | ["captcha", ""] => {
            let svg_base64 = general_purpose::STANDARD.encode(svg.as_bytes());

            let payload = json!({
                "image": format!("data:image/svg+xml;base64,{}", svg_base64),
                "text" : letters
            });

            let mut response = Response::from_json(&payload)?;
            response.headers_mut().append("Content-Type", "application/json")?;

            Ok(response)
        }
        _ => {
            let response = Response::error("Invalid action!", 404)?;
            Ok(response)
        }
    }
}

fn random_range(min: f32, max: f32, byte: u8) -> f32 {
    let norm = byte as f32 / 255.0;
    min + norm * (max - min)
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

fn generate_noise_lines(count: usize, width: f32, height: f32, min_width: f32, max_width: f32) -> String {
    let mut bytes = vec![0u8; count * 6];
    fill(&mut bytes).expect("Failed to get random bytes for lines");

    let mut lines = String::new();

    for i in 0..count {
        let x1 = (bytes[i * 6] as f32 / 255.0) * width;
        let y1 = (bytes[i * 6 + 1] as f32 / 255.0) * height;
        let x2 = (bytes[i * 6 + 2] as f32 / 255.0) * width;
        let y2 = (bytes[i * 6 + 3] as f32 / 255.0) * height;
        let stroke_width = min_width + (bytes[i * 6 + 4] as f32 / 255.0) * (max_width - min_width);
        let opacity = 0.1 + (bytes[i * 6 + 5] as f32 / 255.0) * 0.2;

        lines.push_str(&format!(
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="black" stroke-opacity="{:.2}" stroke-width="{:.1}" />"#,
            x1, y1, x2, y2, opacity, stroke_width
        ));
    }

    lines
}

fn generate_noise_paths(count: usize, width: f32, height: f32, min_stroke: f32, max_stroke: f32, ctrl_height_range: f32, max_path_length: f32) -> String {
    let mut bytes = vec![0u8; count * 9];
    fill(&mut bytes).expect("Failed to get random bytes for paths");

    let mut paths = String::new();

    for i in 0..count {
        let x1 = (bytes[i * 9] as f32 / 255.0) * width;
        let y1 = (bytes[i * 9 + 1] as f32 / 255.0) * height;

        // Calcolo spostamento
        let angle = (bytes[i * 9 + 2] as f32 / 255.0) * std::f32::consts::TAU; // angolo tra 0 e 2π
        let dist = (bytes[i * 9 + 3] as f32 / 255.0) * max_path_length;

        let x2 = (x1 + dist * angle.cos()).clamp(0.0, width);
        let y2 = (y1 + dist * angle.sin()).clamp(0.0, height);

        let ctrl_x = (bytes[i * 9 + 4] as f32 / 255.0) * width;
        let ctrl_y = (bytes[i * 9 + 5] as f32 / 255.0) * ctrl_height_range;

        let stroke_byte = bytes[i * 9 + 6];
        let stroke_width = min_stroke + (stroke_byte as f32 / 255.0) * (max_stroke - min_stroke);
        let opacity = 0.1 + (bytes[i * 9 + 7] as f32 / 255.0) * 0.2;

        paths.push_str(&format!(
            r#"<path d="M {:.1} {:.1} Q {:.1} {:.1}, {:.1} {:.1}" stroke="black" stroke-opacity="{:.2}" stroke-width="{:.1}" fill="none" />"#,
            x1, y1, ctrl_x, ctrl_y, x2, y2, opacity, stroke_width
        ));
    }

    paths
}

fn generate_noise_circles(count: usize, width: f32, height: f32, min_radius: f32, max_radius: f32) -> String {
    let mut bytes = vec![0u8; count * 5];
    fill(&mut bytes).expect("Failed to get random bytes for circles");

    let mut circles = String::new();

    for i in 0..count {
        let cx = (bytes[i * 5] as f32 / 255.0) * width;
        let cy = (bytes[i * 5 + 1] as f32 / 255.0) * height;
        let r = min_radius + (bytes[i * 5 + 2] as f32 / 255.0) * (max_radius - min_radius);
        let opacity = 0.05 + (bytes[i * 5 + 3] as f32 / 255.0) * 0.2;
        let gray = bytes[i * 5 + 4];

        circles.push_str(&format!(
            r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="rgb({},{},{})" fill-opacity="{:.2}" />"#,
            cx, cy, r, gray, gray, gray, opacity
        ));
    }

    circles
}

fn generate_svg(word: &str) -> String {
    let mut rand_bytes = vec![0u8; word.len() * 6];
    fill(&mut rand_bytes).expect("Failed to get randomness");

    let canvas_width = 250.0;
    let canvas_height = 50.0;
    let center_y = canvas_height / 2.0;
    let spacing = 10.0;

    let mut total_width = 0.0;
    let mut dimensions = Vec::new();
    let filters = ["blur-sm", "blur-md", "blur-lg"];

    for i in 0..word.len() {
        let font_size = random_range(18.0, 30.0, rand_bytes[i * 4]);
        let scale_x = random_range(0.8, 1.2, rand_bytes[i * 4 + 1]);
        let scale_y = random_range(0.8, 1.2, rand_bytes[i * 4 + 2]);
        let rotate = random_range(-15.0, 15.0, rand_bytes[i * 4 + 3]);
        let opacity = random_range(0.3, 1.0, rand_bytes[i * 4 + 4]);

        let filter_id = filters[rand_bytes[i] as usize % filters.len()];

        let base_width = font_size * scale_x;
        let rotation_factor = 1.0 + (rotate.to_radians().sin().abs() * 0.5);
        let letter_width = base_width * rotation_factor + spacing;

        dimensions.push((font_size, scale_x, scale_y, rotate, letter_width, opacity, filter_id));
        total_width += letter_width;
    }

    let mut x_pos = (canvas_width - total_width) / 2.0;
    let mut svg_letters = String::new();

    for (i, char) in word.chars().enumerate() {
        let (font_size, scale_x, scale_y, rotate, letter_width, opacity, filter_id) = dimensions[i];

        svg_letters.push_str(&format!(
            r#"<g 
                transform="translate({x_pos:.1}, {center_y:.1}) 
                rotate({rotate:.1}) 
                scale({scale_x:.2},{scale_y:.2})"
                filter="url(#{filter_id})">
                <text 
                    x="0"
                    y="0"
                    font-family="Arial" 
                    font-size="{font_size:.1}" 
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

    let noise_lines = generate_noise_lines(4, canvas_width, canvas_height, 1.0, 3.0);
    let noise_circles = generate_noise_circles(10, canvas_width, canvas_height, 1.0, 3.0);
    let noise_paths = generate_noise_paths(3, canvas_width, canvas_height, 1.0, 3.0, canvas_height, 60.0);

    SVG_TEMPLATE
        .replace("{{letters}}", &svg_letters)
        .replace("{{lines}}", &noise_lines)
        .replace("{{circles}}", &noise_circles)
        .replace("{{paths}}", &noise_paths)
}