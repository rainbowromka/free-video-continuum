use crate::ui::elements::widget::Widget;


pub struct Ui {
    elements: Vec<Box<dyn Widget>>,
    new_elements: Vec<Box<dyn Widget>>,
    client_width: u32,
    client_height: u32,
    bg_color: (f32, f32, f32),
}

impl Ui {
    pub fn new(client_width: u32, client_height: u32) -> Self {
        Self {
            elements: Vec::new(),
            new_elements: Vec::new(),
            client_width,
            client_height,
            bg_color: hex_to_rgb(0x16, 0x19, 0x20),
        }
    }
    
    pub fn update_size(&mut self, w: u32, h: u32) {
        self.client_width = w;
        self.client_height = h;
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.update_size(w, h);

        // Помечаем всё рекурсивно dirty
        for element in &mut self.elements {
            element.mark_dirty_recursive();
        }
        for element in &mut self.new_elements {
            element.mark_dirty_recursive();
        }
    }

    pub fn client_width(&self) -> u32 {
        self.client_width
    }

    pub fn client_height(&self) -> u32 {
        self.client_height
    }

    pub fn render(&mut self) {
        let new_count = self.new_elements.len();

        // Проходим по новым Rect'ам
        for (index, new_element) in self.new_elements.drain(..).enumerate() {
            if index < self.elements.len() {
                let old = &mut self.elements[index];

                if old.as_any().type_id() == new_element.as_any().type_id() {
                    old.update_from(new_element.as_ref());
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
        self.draw();

        self.new_elements.clear();
    }

    pub fn draw(&mut self) {
        // Сначала подготовили текстуры
        for element in &mut self.elements {
            element.create_textures();
        }

        let bg = self.bg_color();
        unsafe {
            gl::Viewport(0, 0, self.client_width as i32, self.client_height as i32);
            gl::ClearColor(bg.0, bg.1, bg.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Потом отрисовали
        for element in &mut self.elements {
            element.draw(self.client_width, self.client_height);
        }
    }

    pub fn set_client_color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.bg_color = hex_to_rgb(r, g, b);
        self
    }

    pub fn bg_color(&self) -> (f32, f32, f32) {
        self.bg_color
    }

    pub fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static,
    {
        let mut element = T::default();
        configure(&mut element);
        self.new_elements.push(Box::new(element));
        self
    }
}

fn hex_to_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

// impl AddElement for Ui {
//     fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
//     where
//         T: Widget + Default + 'static,
//     {
//         let mut element = T::default();
//         configure(&mut element);
//         self.new_elements.push(Box::new(element));
//         self
//     }
// }