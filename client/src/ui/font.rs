use rusttype::{Font, Scale};
use lazy_static::lazy_static;

pub struct FontManager {
    font: Font<'static>,
}

impl FontManager {
    pub fn new() -> Self {
        let font_data: &[u8] = include_bytes!("../../assets/DejaVuSansMono.ttf");
        let font = Font::try_from_bytes(font_data)
            .expect("Failed to parse font");

        Self { font }
    }

    pub fn measure_text(&self, text: &str, size: f32) -> (u32, u32) {
        let scale = Scale::uniform(size);
        let v_metrics = self.font.v_metrics(scale);
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        let width = self.font
            .layout(text, scale, rusttype::point(0.0, v_metrics.ascent))
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0)
            .ceil() as u32;

        (width, height)
    }

    pub fn rasterize_text(&self, text: &str, size: f32) -> (Vec<u8>, u32, u32) {
        let (width, height) = self.measure_text(text, size);

        let mut bitmap = vec![0u8; (width * height * 4) as usize];
        let scale = Scale::uniform(size);
        let v_metrics = self.font.v_metrics(scale);

        for glyph in self.font.layout(text, scale, rusttype::point(0.0, v_metrics.ascent)) {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, coverage| {
                    let px = (bounding_box.min.x + x as i32) as u32;
                    let py = (bounding_box.min.y + y as i32) as u32;

                    if px < width && py < height {
                        let idx = ((py * width + px) * 4) as usize;
                        bitmap[idx + 0] = 255;
                        bitmap[idx + 1] = 255;
                        bitmap[idx + 2] = 255;
                        bitmap[idx + 3] = (coverage * 255.0) as u8;
                    }
                });
            }
        }

        (bitmap, width, height)
    }
}


lazy_static! {
    pub static ref FONT_MANAGER: FontManager = FontManager::new();
}