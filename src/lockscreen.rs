use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ConnectionExt, CreateGCAux, CreateWindowAux, EventMask, Font, Gcontext, GrabMode, Screen,
    Window, WindowClass,
};
use x11rb::rust_connection::RustConnection;
use x11rb::CURRENT_TIME;

use crate::util::measure_text_width;

pub struct Lockscreen<'a> {
    screen: &'a Screen,
    conn: &'a RustConnection,
    font: Font,
    gc: Gcontext,
    window: Window,
}

impl<'a> Lockscreen<'a> {
    pub fn new(
        screen: &'a Screen,
        conn: &'a RustConnection,
        font: Font,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = conn.generate_id()?;
        let gc = conn.generate_id()?;

        conn.create_window(
            screen.root_depth,
            window,
            screen.root,
            0,
            0,
            screen.width_in_pixels,
            screen.height_in_pixels,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new()
                .background_pixel(screen.black_pixel)
                .event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE)
                .override_redirect(1),
        )
        .expect("could not create lockscreen window");

        conn.create_gc(
            gc,
            window,
            &CreateGCAux::default()
                .foreground(screen.white_pixel)
                .graphics_exposures(0)
                .font(font),
        )
        .expect("could not create graphics context");

        conn.map_window(window)?;
        conn.grab_keyboard(true, window, CURRENT_TIME, GrabMode::ASYNC, GrabMode::ASYNC)
            .expect("could not grab keyboard");
        conn.flush()?;

        Ok(Self {
            screen,
            font,
            conn,
            gc,
            window,
        })
    }

    pub fn message(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.clear()?;

        let width = measure_text_width(self.conn, self.font, text)?;
        let text_x = ((self.screen.width_in_pixels as i16 - width) / 2) as i16;
        let text_y = (self.screen.height_in_pixels / 2) as i16;

        self.conn
            .image_text8(self.window, self.gc, text_x, text_y, text.as_bytes())?;

        self.conn.flush()?;

        Ok(())
    }

    pub fn show_stars(&self, pass: &str) {
        let stars: String = pass.chars().into_iter().map(|_| "*").collect();
        self.message(&stars).unwrap();
    }

    fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.clear_area(
            true,
            self.window,
            0,
            0,
            self.screen.width_in_pixels,
            self.screen.height_in_pixels,
        )?;

        Ok(())
    }
}

impl<'a> Drop for Lockscreen<'a> {
    fn drop(&mut self) {
        self.conn.free_gc(self.gc).unwrap();
    }
}
