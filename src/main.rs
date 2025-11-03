pub mod args;
pub mod tui;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = args::opts().run();

    match opts.command {
        args::Command::Tui => tui::ui::run(),
        args::Command::Web => return Ok(()),
    }
}
