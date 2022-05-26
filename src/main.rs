mod locker;
mod lockscreen;
mod util;

use locker::Locker;

use clap::Parser;

/// Simple x lockscreen
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Locked message
    #[clap(short, long, default_value_t = String::from("locked"))]
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (conn, _) = x11rb::connect(None)?;
    Locker::new(&conn).start(&args.message)?;

    Ok(())
}
