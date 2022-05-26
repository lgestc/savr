use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{ConnectionExt, Font},
        Event,
    },
    rust_connection::RustConnection,
};
use xkbcommon::xkb::{self, keysym_to_utf8, KEY_BackSpace, KEY_Escape, KEY_Return};

use crate::{
    lockscreen::Lockscreen,
    util::{check_password, get_username},
};

pub struct Locker<'a> {
    locks: Vec<Lockscreen<'a>>,
    conn: &'a RustConnection,
    font: Font,
}

impl<'a> Locker<'a> {
    pub fn new(conn: &'a RustConnection) -> Self {
        let screens = &conn.setup().roots;
        let font = conn.generate_id().unwrap();
        conn.open_font(font, "fixed".as_bytes())
            .expect("could not open font");

        let locks: Vec<Lockscreen> = screens
            .iter()
            .map(|screen| Lockscreen::new(&screen, &conn, font).unwrap())
            .collect();

        Self { locks, conn, font }
    }

    fn message(&self, message: &str) {
        self.locks
            .iter()
            .for_each(|lock| lock.message(message).unwrap());
    }

    fn show_stars(&self, pass: &str) {
        self.locks.iter().for_each(|lock| lock.show_stars(pass));
    }

    pub fn start(&self, locked_message: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let failed_message = "failed";
        let verifying_message = "verifying";

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_names(
            &context,
            "", // rules
            "", // model
            "", // layout
            "", // variant
            None,
            xkb::COMPILE_NO_FLAGS,
        )
        .unwrap();
        let mut state = xkb::State::new(&keymap);
        let mut pass = String::new();

        loop {
            let evt = self.conn.wait_for_event()?;

            match evt {
                Event::KeyPress(key) => {
                    let keycode = key.detail as u32;
                    state.update_key(keycode, xkb::KeyDirection::Down);
                    let keysym = state.key_get_one_sym(keycode);

                    match keysym {
                        KEY_Return => {
                            self.message(verifying_message);

                            match check_password(&pass) {
                                Ok(_) => break,
                                Err(_err) => {
                                    self.message(failed_message);
                                }
                            }

                            pass = String::new();
                        }
                        KEY_Escape => {
                            pass = String::new();
                        }
                        KEY_BackSpace => {
                            pass.pop();
                            self.show_stars(&pass);
                        }
                        _ => {
                            let character = keysym_to_utf8(keysym).replace("\0", "");
                            pass.push_str(&character);

                            if pass.len() > 32 {
                                pass = String::new();
                            }

                            self.show_stars(&pass);
                        }
                    }
                }
                Event::KeyRelease(key) => {
                    let keycode = key.detail as u32;
                    state.update_key(keycode, xkb::KeyDirection::Up);

                    let keysym = state.key_get_one_sym(keycode);

                    if keysym == KEY_BackSpace {
                        pass.pop();
                        self.show_stars(&pass);
                    }
                }
                _ => println!("{:?}", evt),
            }

            if pass.len() == 0 {
                self.message(&locked_message);
            }
        }

        Ok(())
    }
}

impl<'a> Drop for Locker<'a> {
    fn drop(&mut self) {
        self.conn.close_font(self.font).unwrap();
    }
}
