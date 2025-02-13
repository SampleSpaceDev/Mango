use crate::types::ui::canvas::{BACKUP_TYPEFACE, MINECRAFT_TYPEFACE};
use crate::types::ui::colors::colors;
use crate::types::ui::colors::colors::{exists, COLORS};
use crate::types::ui::colors::shadows::SHADOWS;
use skia_safe::{Canvas, Color, Color4f, Font, Paint, Typeface};
use std::ptr::eq;

#[derive(Debug)]
enum Node {
    Text(String),
    Tag { tag: String, children: Vec<Node> },
}

fn parse_nodes_rec<'a>(mut input: &'a str, expected: Option<&str>) -> (Vec<Node>, &'a str) {
    let mut nodes = Vec::new();

    while !input.is_empty() {
        if input.starts_with("</") {
            if let Some(end) = input.find('>') {
                let closing_tag = &input[..end + 1];
                if let Some(exp) = expected {
                    if closing_tag == exp {
                        input = &input[end + 1..];
                        return (nodes, input);
                    } else {
                        nodes.push(Node::Text(closing_tag.to_string()));
                        input = &input[end + 1..];
                        continue;
                    }
                } else {
                    nodes.push(Node::Text(closing_tag.to_string()));
                    input = &input[end + 1..];
                    continue;
                }
            } else {
                nodes.push(Node::Text(input.to_string()));
                input = "";
                break;
            }
        } else if input.starts_with("<") {
            if let Some(end) = input.find('>') {
                let tag = &input[1..end];
                input = &input[end + 1..];

                let expected_closing = format!("</{}>", tag);

                let (children, new_input) = parse_nodes_rec(input, Some(&expected_closing));
                input = new_input;

                nodes.push(Node::Tag { tag: tag.to_string(), children });
            } else {
                nodes.push(Node::Text(input.to_string()));
                input = "";
                break;
            }
        } else {
            if let Some(idx) = input.find('<') {
                let text = &input[..idx];
                nodes.push(Node::Text(text.to_string()));
                input = &input[idx..];
            } else {
                nodes.push(Node::Text(input.to_string()));
                input = "";
            }
        }
    }

    (nodes, input)
}

fn parse_nodes(input: &str) -> Vec<Node> {
    let (nodes, _) = parse_nodes_rec(input, None);
    nodes
}

fn flatten_nodes(nodes: &[Node], current_color: Option<Color>) -> Vec<(String, Option<Color>)> {
    let mut segments = Vec::new();

    for node in nodes {
        match node {
            Node::Text(text) => {
                segments.push((text.clone(), current_color));
            }
            Node::Tag { tag, children } => {
                let new_color = parse_color(tag).or(current_color);
                let child_segments = flatten_nodes(children, new_color);
                segments.extend(child_segments);
            }
        }
    }

    segments
}

fn parse_and_flatten(input: &str) -> Vec<(String, Option<Color>)> {
    let nodes = parse_nodes(input);
    flatten_nodes(&nodes, None)
}

fn parse_color(tag: &str) -> Option<Color> {
    if tag.starts_with('#') {
        let hex = &tag[1..];
        if hex.len() == 6 {
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;

                return Some(Color::from_argb(255, r, g, b))
            }
        } else if hex.len() == 8 {
            if let Ok(argb) = u32::from_str_radix(hex, 16) {
                let a = ((argb >> 24) & 0xFF) as u8;
                let r = ((argb >> 16) & 0xFF) as u8;
                let g = ((argb >> 8) & 0xFF) as u8;
                let b = (argb & 0xFF) as u8;
                
                return Some(Color::from_argb(a, r, g, b))
            }
        }
        None
    } else {        
        colors::get_color(tag.to_lowercase().as_str()).map(|hex| {
            let rgb = u32::from_str_radix(&hex[1..], 16).unwrap();
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            
            Color::from_argb(255, r, g, b)
        })
    }
}

pub struct TextRenderer<'a> {
    canvas: &'a Canvas,
}

impl<'a> TextRenderer<'a> {
    pub fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }
    
    pub fn draw_text(&mut self, input: &str, mut x: f32, y: f32, text_size: f32, shadow: bool) {
        let runs = extract_text_runs(input);
        
        let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
        paint.set_anti_alias(true);
        
        for run in runs {
            let font = Font::new(run.typeface, text_size * run.scale);

            if let Some(color) = run.color {
                paint.set_color(color);
            } else {
                paint.set_color(Color::WHITE);
            }

            if shadow {
                let offset = if text_size >= 25.0 { 4.0 } else { 2.0 };
                
                let shadow_color = get_shadow(paint.color());
                let mut shadow_paint = paint.clone();
                shadow_paint.set_color(shadow_color);
                
                self.canvas.draw_str(&run.text, (x + offset, y + offset), &font, &shadow_paint);
            }
            
            self.canvas.draw_str(&run.text, (x, y), &font, &paint);
            
            let (width, _) = font.measure_str(&run.text, Some(&paint));
            x += width as f32;
        }
    }
}

struct TextRun {
    text: String,
    typeface: Typeface,
    scale: f32,
    color: Option<Color>,
}

fn extract_text_runs(input: &str) -> Vec<TextRun> {
    let segments = parse_and_flatten(input);
    let mut runs = Vec::new();
    
    for (segment, color_opt) in segments {
        if segment.is_empty() {
            continue;
        }
        
        let mut run = String::new();
        let mut current_typeface = &*MINECRAFT_TYPEFACE;
        let mut current_scale: f32 = 1.0;
        
        for ch in segment.chars() {
            let main_glyph = current_typeface.unichar_to_glyph(ch as i32);
            let (desired_typeface, desired_scale) = if main_glyph == 0 {
                (&*BACKUP_TYPEFACE, get_scale_factor(ch))
            } else {
                (&*MINECRAFT_TYPEFACE, 1.0)
            };
            
            if !eq(desired_typeface, current_typeface)
                || (desired_scale != current_scale)
                && !run.is_empty() 
            {                    
                runs.push(TextRun {
                    text: run.clone(),
                    typeface: current_typeface.clone(),
                    scale: current_scale,
                    color: color_opt,
                });
                
                run.clear();
            }
            
            current_typeface = desired_typeface;
            current_scale = desired_scale;
            run.push(ch);
        }
        
        if !run.is_empty() {
            runs.push(TextRun {
                text: run,
                typeface: current_typeface.clone(),
                scale: current_scale,
                color: color_opt,
            });
        }
    }
    
    runs
}

pub struct TextMetrics {
    pub width: f32,
    pub ascent: f32,
    pub descent: f32,
}

pub fn measure(text: &str, text_size: f32) -> TextMetrics {
    let runs = extract_text_runs(text);
    
    let mut total_advance: f32 = 0.0;
    let mut max_ascent: f32 = 0.0;
    let mut max_descent: f32 = 0.0;
    
    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    paint.set_anti_alias(true);
    
    for run in runs {
        let font = Font::new(run.typeface, text_size * run.scale);
        
        let (advance, bounds) = font.measure_str(&run.text, Some(&paint));
        
        total_advance += advance;
        max_ascent = max_ascent.min(bounds.top);
        max_descent = max_descent.max(bounds.bottom);
    }

    TextMetrics {
        width: total_advance,
        ascent: max_ascent,
        descent: max_descent,
    }
}

fn get_scale_factor(ch: char) -> f32 {    
    match ch {
        '✫' | '✪' | '⚝' | '✥' => 0.8,
        _ => 1.0,
    }
}

fn color_to_hex(color: Color) -> String {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn hex_to_color(hex: &str) -> Color {
    let rgb = u32::from_str_radix(&hex[1..], 16).unwrap();
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    
    Color::from_argb(255, r, g, b)
}

pub fn get_shadow(color: Color) -> Color {
    let hex = color_to_hex(color);
    
    if exists(&hex) {
        let index = COLORS.iter().position(|(_, v)| *v == hex).unwrap();
        let (_, shadow_hex) = SHADOWS[index];
        
        hex_to_color(shadow_hex)
    } else {
        let factor = 0.75;
        let new_color = darken_color(color, factor);
        
        new_color
    }
}

fn darken_color(color: Color, factor: f32) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();
    
    let adjust = |component: u8| -> u8 {
        let comp = component as f32;
        
        let new_comp = if factor >= 0.0 {
            comp * (1.0 - factor)       
        } else {
            comp + (255.0 - comp) * (-factor)
        };
        
        new_comp.round().clamp(0.0, 255.0) as u8
    };
    
    Color::from_rgb(adjust(r), adjust(g), adjust(b))
}