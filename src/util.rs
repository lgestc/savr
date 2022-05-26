use std::process::Command;

use x11rb::{
    protocol::xproto::{Char2b, ConnectionExt, Font},
    rust_connection::RustConnection,
};

pub fn get_username() -> Result<String, Box<dyn std::error::Error>> {
    let user = String::from_utf8(Command::new("whoami").output()?.stdout)?;
    Ok(user.trim().to_string())
}

pub fn check_password(password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let service = "system-auth";
    let user = get_username()?;

    let mut auth = pam::Authenticator::with_password(service)?;
    auth.get_handler().set_credentials(user, password);

    match auth.authenticate() {
        Ok(_) => Ok(true),
        Err(err) => Err(err.into()),
    }
}

pub fn measure_text_width(
    conn: &RustConnection,
    font: Font,
    text: &str,
) -> Result<i16, Box<dyn std::error::Error>> {
    let text_as_char2b: Vec<Char2b> = text
        .as_bytes()
        .to_vec()
        .iter()
        .map(|b| Char2b {
            byte1: *b,
            byte2: *b,
        })
        .collect();

    let width = conn
        .query_text_extents(font, &text_as_char2b)?
        .reply()
        .expect("could not obtain font information")
        .overall_width as i16;

    Ok(width)
}
