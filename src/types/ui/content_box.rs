use crate::types::ui::canvas::Canvas;
use crate::types::ui::text_renderer::measure;
use crate::COLOR_REGEX;
use skia_safe::{Color, Data, Image, PaintStyle, Point};

pub struct ContentBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shapes: Vec<ShapeContent>,
    texts: Vec<TextContent>,
    images: Vec<ImageContent>,
    tables: Vec<TableContent>,
    content_boxes: Vec<ContentBox>,
    padding: f32,
}

impl ContentBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            shapes: Vec::new(),
            texts: Vec::new(),
            images: Vec::new(),
            tables: Vec::new(),
            content_boxes: Vec::new(),
            padding: 0.0,
        }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        let background = ShapeContent {
            color,
            ..ShapeContent::rounded_rect(0.0, 0.0, self.width, self.height, 5.0, 5.0)
        };

        self.shapes.insert(0, background);
        self
    }

    pub fn with_border(mut self, color: Color) -> Self {
        let border = ShapeContent {
            style: PaintStyle::Stroke,
            color,
            ..ShapeContent::rounded_rect(0.0, 0.0, self.width, self.height, 5.0, 5.0)
        };

        self.shapes.push(border);
        self
    }

    pub fn add_text(mut self, text: TextContent) -> Self {
        let text_with_padding = TextContent {
            x: text.x + self.padding,
            y: text.y + self.padding,
            ..text
        };

        self.texts.push(text_with_padding);
        self
    }

    pub fn add_shape(mut self, shape: ShapeContent) -> Self {
        let shape_with_padding = ShapeContent {
            x: shape.x + self.padding,
            y: shape.y + self.padding,
            ..shape
        };
        
        self.shapes.push(shape_with_padding);
        self
    }
    
    pub fn add_image(mut self, image: ImageContent) -> Self {
        let image_with_padding = ImageContent {
            x: image.x + self.padding,
            y: image.y + self.padding,
            ..image
        };
        
        self.images.push(image_with_padding);
        self
    }

    pub fn add_table(mut self, table: TableContent) -> Self {
        let table_with_padding = TableContent {
            x: table.x + self.padding,
            y: table.y + self.padding,
            ..table
        };
        
        self.tables.push(table_with_padding);
        self
    }
    
    pub fn add_content_box(mut self, _box: ContentBox) -> Self {
        let x = _box.x + self.padding;
        let y = _box.y + self.padding;
        let box_with_padding = _box.set_position(x, y);
        
        self.content_boxes.push(box_with_padding);
        self
    }
    
    pub fn add_content_boxes(mut self, boxes: Vec<ContentBox>) -> Self {
        for b in boxes {
            self = self.add_content_box(b);
        }
        
        self
    }
    
    fn set_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        
        self
    }
    
    pub fn render(&mut self, canvas: &mut Canvas) {
        canvas.save();
        canvas.translate(self.x, self.y);

        for shape in &self.shapes {
            shape.render(canvas);
        }

        let inner_width = self.width - self.padding * 2.0;
        let inner_height = self.height - self.padding * 2.0;
        
        for image in &self.images {
            image.render(canvas, inner_width, inner_height);
        }

        for text in &mut self.texts {
            text.render(canvas, inner_width, inner_height);
        }

        for table in &self.tables {
            table.render(canvas);
        }
        
        for _box in &mut self.content_boxes {
            _box.render(canvas);
        }
        
        canvas.restore();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}


/// ShapeContent
pub enum ShapeType {
    Rectangle,
    Circle,
    Line,
    RoundedRectangle,
}

pub struct ShapeContent {
    pub shape_type: ShapeType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub style: PaintStyle,
    pub rx: f32,
    pub ry: f32,
}

impl ShapeContent {
    pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            shape_type: ShapeType::Rectangle,
            x,
            y,
            width,
            height,
            color: Color::BLACK,
            style: PaintStyle::Fill,
            rx: 0.0,
            ry: 0.0,
        }
    }

    pub fn circle(x: f32, y: f32, radius: f32) -> Self {
        Self {
            shape_type: ShapeType::Circle,
            x,
            y,
            width: radius,
            height: radius,
            color: Color::BLACK,
            style: PaintStyle::Fill,
            rx: 0.0,
            ry: 0.0,
        }
    }

    pub fn line(x: f32, y: f32, x2: f32, y2: f32) -> Self {
        Self {
            shape_type: ShapeType::Line,
            x,
            y,
            width: x2,
            height: y2,
            color: Color::BLACK,
            style: PaintStyle::Stroke,
            rx: 0.0,
            ry: 0.0,
        }
    }

    pub fn rounded_rect(x: f32, y: f32, width: f32, height: f32, rx: f32, ry: f32) -> Self {
        Self {
            shape_type: ShapeType::RoundedRectangle,
            x,
            y,
            width,
            height,
            color: Color::BLACK,
            style: PaintStyle::Fill,
            rx,
            ry,
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        let original_color = canvas.paint.color();
        canvas.set_color(self.color);

        let original_style = canvas.paint.style();
        canvas.set_style(self.style);

        match self.shape_type {
            ShapeType::Rectangle => {
                canvas.draw_rect(self.x, self.y, self.width, self.height);
            }
            ShapeType::Circle => {
                canvas.draw_circle(self.x, self.y, self.width);
            }
            ShapeType::Line => {
                canvas.move_to(self.x, self.y);
                canvas.line_to(self.x + self.width, self.y + self.height);
                canvas.stroke();
            }
            ShapeType::RoundedRectangle => {
                canvas.draw_rounded_rect(self.x, self.y, self.width, self.height, self.rx, self.ry);
            }
        }

        canvas.set_color(original_color);
        canvas.set_style(original_style);
    }
}


/// ImageContent
pub struct ImageContent {
    pub bytes: Vec<u8>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub alignment: Alignment,
    pub vertical_alignment: VerticalAlignment
}

impl ImageContent {
    pub fn new(bytes: Vec<u8>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bytes,
            x,
            y,
            width,
            height,
            scale: 1.0,
            alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
    
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
    
    pub fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
    
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    
    pub fn render(&self, canvas: &mut Canvas, inner_width: f32, inner_height: f32) {
        let image = Image::from_encoded(Data::new_copy(&self.bytes)).expect("Failed to decode image.");
        
        canvas.save();
        canvas.scale(self.scale, self.scale);
        
        let adjusted_x = match self.alignment {
            Alignment::Left => self.x,
            Alignment::Center => (inner_width - image.width() as f32) / 2.0 + self.x,
            Alignment::Right => inner_width - (image.width() as f32) - self.x,
        };

        let adjusted_y = match self.vertical_alignment {
            VerticalAlignment::Top =>
                self.y - (image.height() as f32),
            VerticalAlignment::Middle =>
                self.y + (inner_height / 2.0) - (image.height() as f32 / 2.0),
            VerticalAlignment::Bottom =>
                self.y + inner_height - (image.height() as f32),
        };
        
        let position = Point::new(adjusted_x / self.scale, adjusted_y / self.scale);
        canvas.draw_image(image, position);
        
        canvas.restore();
    }
}


/// TextContent
pub struct TextContent {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub shadow: bool,
    pub alignment: Alignment,
    pub vertical_alignment: VerticalAlignment,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left // Set Left as the default
    }
}

impl TextContent {
    pub fn new(text: String, x: f32, y: f32, size: f32) -> Self {
        Self {
            text,
            x,
            y,
            size,
            shadow: true,
            alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    pub fn with_shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    fn render(&mut self, canvas: &mut Canvas, inner_width: f32, inner_height: f32) {
        let modified_text = COLOR_REGEX.replace_all(&self.text, "");
        let metrics = measure(&modified_text, self.size);
        let text_height = metrics.descent - metrics.ascent;
        
        let mut total_width = metrics.width;

        // Decrease text size until the total width fits within max_width
        while total_width > inner_width && self.size > 1.0 {
            self.size -= 1.0;
            total_width = measure(self.text.as_str(), self.size).width;
            if total_width <= inner_width {
                break;
            }
        }

        let adjusted_x = match self.alignment {
            Alignment::Left => self.x,
            Alignment::Center => (inner_width - total_width) / 2.0 + self.x,
            Alignment::Right => inner_width - total_width - self.x,
        };

        let adjusted_y = match self.vertical_alignment {
            VerticalAlignment::Top => 
                self.y - metrics.ascent,
            VerticalAlignment::Middle =>
                self.y + (inner_height / 2.0) - metrics.ascent - (text_height / 2.0),
            VerticalAlignment::Bottom => 
                self.y + inner_height - metrics.descent,
        };
        
        canvas.draw_text(self.text.as_str(), adjusted_x, adjusted_y, self.size, self.shadow);
    }
}

/// TableContent
pub struct TableContent {
    pub x: f32,
    pub y: f32,
    pub columns: usize,
    pub rows: Vec<Vec<String>>,
    pub cell_width: f32,
    pub cell_height: f32,
    pub border_color: Color,
    pub text_size: f32,
}

// #[deprecated(note = "Incomplete. Should not be used.")]
impl TableContent {
    pub fn new(x: f32, y: f32, columns: usize, cell_width: f32, cell_height: f32) -> Self {
        Self {
            x,
            y,
            columns,
            rows: Vec::new(),
            cell_width,
            cell_height,
            border_color: Color::BLACK,
            text_size: 16.0,
        }
    }
    
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    
    pub fn with_text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    
    pub fn add_row(mut self, row: Vec<String>) -> Self {
        if row.len() == self.columns {
            self.rows.push(row);
        }
        self
    }
    
    pub fn render(&self, canvas: &mut Canvas) {
        let mut y_offset = self.y;
        
        for row in &self.rows {
            let mut x_offset = self.x;
            
            for cell in row {
                if !cell.is_empty() {
                    let border = ShapeContent {
                        shape_type: ShapeType::Rectangle,
                        x: x_offset,
                        y: y_offset,
                        width: self.cell_width,
                        height: self.cell_height,
                        color: self.border_color,
                        style: PaintStyle::Stroke,
                        rx: 0.0,
                        ry: 0.0,
                    };

                    border.render(canvas);
                } 
                
                // let text_metrics = measure(cell, self.text_size);
                // let text_x = x_offset + (self.cell_width - text_metrics.width) / 2.0;
                // let text_y = y_offset + (self.cell_height - (text_metrics.descent - text_metrics.ascent));
                
                let mut text_content = TextContent {
                    text: cell.clone(),
                    x: x_offset,
                    y: y_offset,
                    size: self.text_size,
                    shadow: false,
                    alignment: Alignment::Center,
                    vertical_alignment: VerticalAlignment::Middle,
                };
                
                text_content.render(canvas, self.cell_width, self.cell_height);
                
                x_offset += self.cell_width;
            }
            
            y_offset += self.cell_height;
        }
    }
}