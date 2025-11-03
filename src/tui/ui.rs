use color_eyre::Result;

use ratatui::{
    crossterm::event::{self, Event},
    Frame,
};

pub fn run() -> Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello World", frame.area());
}
