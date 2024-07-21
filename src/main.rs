pub mod app;
pub mod file;
pub mod pos;
pub mod state;

use std::env;

use app::App;

fn main() -> std::io::Result<()> {
    // DEVELOPMENT ARGUMENT
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        App::new(None)?.update()?;
    } else {
        App::new(Some(&args[1]))?.update()?;
    }

    Ok(())
}
