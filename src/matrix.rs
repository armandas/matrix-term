use crate::cell::Cell;
use crossterm::style::Stylize;
use crossterm::{cursor, style, terminal, QueueableCommand};
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::io::{Stdout, Write};

pub struct Matrix {
    rng: ThreadRng,
    stdout: Stdout,
    width: usize,
    height: usize,
    buffer: Vec<Vec<Option<Cell>>>,
    max_age: u16,
}

impl Matrix {
    pub fn new(width: usize, height: usize, max_age: u16) -> Self {
        Self {
            rng: thread_rng(),
            stdout: std::io::stdout(),
            width: width / 2,
            height,
            buffer: vec![vec![Option::<Cell>::None; width]; height],
            max_age,
        }
    }

    pub fn begin(&mut self) -> std::io::Result<()> {
        self.stdout.queue(terminal::EnterAlternateScreen)?;
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;
        self.stdout.queue(cursor::Hide)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.age();
        self.spawn_parents();
        self.spawn_children();
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.buffer[y][x] {
                    let c = (255.0
                        * (1.0 - ((self.max_age - cell.age) as f32 / self.max_age as f32)))
                        as u8;
                    let styled =
                        format!("{}", cell.content).with(style::Color::Rgb { r: c, g: c, b: c });
                    print!("{}", styled);
                } else {
                    print!("  ");
                }
            }
            self.stdout.queue(cursor::MoveTo(0, y as u16))?;
        }
        self.stdout.flush()
    }

    fn age(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = &mut self.buffer[y][x] {
                    cell.age += if cell.age < self.max_age { 1 } else { 0 };
                }
            }
        }
    }

    fn spawn_parents(&mut self) {
        let spawn_col = self.buffer[0]
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_none())
            .choose(&mut self.rng)
            .unwrap_or_else(|| {
                self.buffer[0]
                    .iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.unwrap().age == self.max_age)
                    .choose(&mut self.rng)
                    .unwrap()
            })
            .0;
        self.buffer[0][spawn_col] = Some(Cell::new(&mut self.rng));
    }

    fn spawn_children(&mut self) {
        for y in 1..self.height {
            for x in 0..self.width {
                if let Some(cell) = &mut self.buffer[y - 1][x] {
                    if cell.age == 1 {
                        self.buffer[y][x] = Some(Cell::new(&mut self.rng));
                    }
                }
            }
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        self.stdout.queue(terminal::LeaveAlternateScreen).unwrap();
        self.stdout.queue(cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}
