use crate::ui::elements::rect::Rect;
use crate::ui::renderer::Renderer;
use crate::ui::elements::text::Text;

pub struct Ui {
    rects: Vec<Rect>,
    new_rects: Vec<Rect>,
    client_width: u32,
    client_height: u32,
    bg_color: (f32, f32, f32),
}

impl Ui {
    pub fn new(client_width: u32, client_height: u32) -> Self {
        Self {
            rects: Vec::new(),
            new_rects: Vec::new(),
            client_width,
            client_height,
            bg_color: hex_to_rgb(0x16, 0x19, 0x20),
        }
    }

    pub fn resize(&mut self, client_width: u32, client_height: u32) {
        self.client_width = client_width;
        self.client_height = client_height;

        for rect in &mut self.rects {
            rect.mark_quad_dirty();
        }
        for rect in &mut self.new_rects {
            rect.mark_quad_dirty();
        }
    }

    pub fn client_width(&self) -> u32 {
        self.client_width
    }

    pub fn client_height(&self) -> u32 {
        self.client_height
    }

    pub fn add_rect<F>(&mut self, x: u32, y: u32, width: u32, height: u32, configure: F) -> &mut Self
    where
        F: FnOnce(&mut Rect),
    {
        let mut rect = Rect::new(x, y, width, height);
        configure(&mut rect);
        self.new_rects.push(rect);
        self
    }

    pub fn add_text<F>(&mut self, text: &str, configure: F) -> &mut Self
    where
        F: FnOnce(&mut Text),
    {
        let mut t = Text::new(text);
        configure(&mut t);
        // TODO: добавить в коллекцию текстов
        self
    }

    pub fn push_rect(&mut self, rect: Rect) {
        self.new_rects.push(rect);
    }

    pub fn render(&mut self, renderer: &Renderer) {
        let new_count = self.new_rects.len();

        // Проходим по новым Rect'ам
        for (index, new_rect) in self.new_rects.drain(..).enumerate() {
            if index < self.rects.len() {
                // Есть старый Rect — сравниваем
                let old_rect = &mut self.rects[index];

                if old_rect != &new_rect {
                    // Что-то изменилось — обновляем только изменения
                    old_rect.update_from(new_rect);
                }
                // Если равны — старый остаётся без изменений
            } else {
                // Новый Rect — добавляем
                self.rects.push(new_rect);
            }
        }

        // Удаляем лишние старые Rect'ы (если новых меньше)
        self.rects.truncate(new_count);

        // Рендерим
        for rect in &mut self.rects {
            rect.draw(
                renderer.texture_program(),
                self.client_width,
                self.client_height,
            );
        }

        self.new_rects.clear();
    }

    pub fn set_client_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.bg_color = hex_to_rgb(r, g, b);
        self
    }

    pub fn bg_color(&self) -> (f32, f32, f32) {
        self.bg_color
    }
}

fn hex_to_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
