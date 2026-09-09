use std::any::Any;

use crate::ui::{elements::{base::BaseElement, widget::Widget}};

pub struct Ui {
    base: BaseElement,
    elements: Vec<Box<dyn Widget>>,
    new_elements: Vec<Box<dyn Widget>>,
    // client_width: u32,
    // client_height: u32,
    // bg_color: (f32, f32, f32),
}

impl Ui {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            base: BaseElement::new(0, 0, width, height),
            elements: Vec::new(),
            new_elements: Vec::new(),
        }
    }

    pub fn update_size(&mut self, w: u32, h: u32) {
        self.base.width = w;
        self.base.height = h;
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.update_size(w, h);
        self.mark_dirty_recursive();
    }

    pub fn client_width(&self) -> u32 {
        self.base.width
    }

    pub fn client_height(&self) -> u32 {
        self.base.height
    }

    pub fn render(&mut self) {
        // Сначала подготовили текстуры
        self.create_textures();

        self.draw(self.base.width, self.base.height);
    }

    pub fn draw_all(&mut self)
    {
        let new_count = self.new_elements.len();

        // Проходим по новым Rect'ам
        for (index, new_element) in self.new_elements.drain(..).enumerate() {
            if index < self.elements.len() {
                let old = &mut self.elements[index];

                if old.as_any().type_id() == new_element.as_any().type_id() {
                    old.diff(new_element.as_ref());
                } else {                
                    *old = new_element;
                }                
            } else {
                // Новый Rect — добавляем
                self.elements.push(new_element);
            }
        }

        // Удаляем лишние старые Rect'ы (если новых меньше)
        self.elements.truncate(new_count);

        // Рендерим
        self.render();

        self.new_elements.clear();
    }

    pub fn set_client_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.base.color = hex_to_rgb(r, g, b);
        self
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        // TODO: если найден элемент — проверяем hover,
        // если hover есть — запускаем обработчик
    }
}

impl Widget for Ui {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn base(&mut self) -> &mut BaseElement {
        &mut self.base
    }    

    fn set_position(&mut self, x: u32, y: u32) {
    }

    fn diff(&mut self, other: &dyn Widget) {
    }

    fn mark_dirty_recursive(&mut self) {
        self.base.mark_dirty();
        for element in &mut self.elements {
            element.mark_dirty_recursive();
        }
        for element in &mut self.new_elements {
            element.mark_dirty_recursive();
        }
    }

    fn create_textures(&mut self) -> bool {
        for element in &mut self.elements {
            element.create_textures();
        }
        true
    }

    fn draw(&mut self, parent_w: u32, parent_h: u32) {
        let bg = self.base.color;
        unsafe {
            gl::Viewport(0, 0, self.base.width as i32, self.base.height as i32);
            gl::ClearColor(bg.0, bg.1, bg.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Потом отрисовали
        for element in &mut self.elements {
            element.draw(parent_w, parent_h);
        }        
    }

    fn push_child(&mut self, child: Box<dyn Widget>) {
        self.new_elements.push(child);
    }

    fn layout(&mut self) {        
    }

    fn prepare_default(&mut self, parent: &mut dyn Widget) {        
    }
}

fn hex_to_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
