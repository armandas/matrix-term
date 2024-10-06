use crossterm::{event, terminal};
use matrix::matrix::Matrix;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let size = crossterm::terminal::size()?;
    let mut matrix = Matrix::new(size.0 as usize, size.1 as usize, 20);

    matrix.begin()?;

    loop {
        matrix.update();
        matrix.render()?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            match event::read()? {
                event::Event::Key(event) => {
                    if event.code == event::KeyCode::Char('q') {
                        break;
                    }
                }
                _ => (),
            };
        }
    }

    Ok(())
}
