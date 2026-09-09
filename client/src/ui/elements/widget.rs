use std::any::Any;

use crate::ui::{elements::base::BaseElement};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;    
    fn base(&mut self) -> &mut BaseElement;
    fn set_position(&mut self, x: u32, y: u32);
    fn diff(&mut self, other: &dyn Widget);
    fn mark_dirty_recursive(&mut self);
    fn create_textures(&mut self) -> bool;
    fn draw(&mut self, parent_w: u32, parent_h: u32);
    fn layout(&mut self);
    fn push_child(&mut self, child: Box<dyn Widget>);

    fn prepare_default(&mut self, parent: &mut dyn Widget) {
        let parent_base = parent.base();
        let color = parent_base.color();
        let z = parent_base.z + 1;

        let base = self.base();

        let abs_x = parent_base.abs_x + base.abs_x;
        let abs_y = parent_base.abs_y + base.abs_y;

        base.set_color_raw(color.0, color.1, color.2);
        base.z = z;
        base.abs_x = abs_x;
        base.abs_y = abs_y;
    }

    fn add<T>(&mut self, configure: impl FnOnce(&mut T)) -> &mut Self
    where
        T: Widget + Default + 'static,
        Self: Sized,
    {
        let mut element = T::default();
        element.prepare_default(self);
        configure(&mut element);
        element.layout();
        // element.register_events(self.base().ui_id);
        self.push_child(Box::new(element));
        self
    }
}
