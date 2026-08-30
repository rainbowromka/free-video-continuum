mod ui;

use ui::window::Window;

use crate::ui::ui::Ui;

fn main() {
    Window::new()
        .with_size(1024, 780)
        .with_title("Free Video Continuum")
        .set_client_color(0x17, 0x1a, 0x21)                
        .on_draw(|ui: &mut Ui| {

            let w = ui.client_width();
            let h = ui.client_height();

           ui.add_rect(0, 0, 390, h)
                .set_color(0x27, 0x2a, 0x31)
                // .content(|ui| {
                //     ui.add_text("[d]disk1")
                //         .set_background_color(0x27, 0x2a, 0x31)
                //         .set_text_color(0xff, 0xff, 0xff)
                //     .build()
                //         .add_text("[r]mamay")
                //     .build()
                // })
            .build()
                .add_rect(w - 370, 0, 370, h)
                .set_color(0x27, 0x2a, 0x31)
            .build();

            //  ui.add_rect(0, 0, 390, h, |rect| {
            //     rect
            //         .set_color(0x27, 0x2a, 0x31)
            //         .content(|ui| {
            //             ui
            //             .add_text("[d]disk1", |text| {
            //                 text
            //                 .set_background_color(0x27, 0x2a, 0x31)
            //                 .set_text_color(0xff, 0xff, 0xff)
            //             })
            //             .add_text("[r]mamay")
            //     })
            //  })
            //  .add_rect(w - 370, 0, 370, h, |rect| {
            //     rect.set_color(0x27, 0x2a, 0x31)
            //  })

        })
        .run();
}