// Reference: https://zenn.dev/tipstar0125/articles/d2cf0ef63bceb7
#![allow(non_snake_case, dead_code)]

use svg::node::element::Circle;
use svg::node::element::Line;
use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::node::Text as TextContent;
use svg::Document;

pub fn document(width: usize, height: usize) -> Document {
    Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height)
        .set("id", "vis")
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn rect_with_stroke(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
        .set("stroke", "black")
        .set("stroke-width", 1)
}

pub fn circle(x: usize, y: usize, r: usize, fill: &str) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("fill", fill)
}

pub fn line(x1: usize, y1: usize, x2: usize, y2: usize, width: usize, color: &str) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", color)
        .set("stroke-width", width)
        .set("stroke-linecap", "round")
}

pub fn text(x: usize, y: usize, font_size: usize, text: &str) -> Text {
    Text::new()
        .add(TextContent::new(text))
        .set("x", x)
        .set("y", y)
        .set("fill", "black")
        .set("font-size", font_size)
        .set("dominant-baseline", "central") // 上下中央揃え
        .set("text-anchor", "middle") // 左右中央揃え
}

pub fn color(mut val: f64) -> String {
    val = val.min(1.0);
    val = val.max(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}
