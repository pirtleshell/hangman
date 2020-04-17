use std::fs::File;
use std::io::prelude::*;

struct HangmanCanvas {
    grid: [[char; 14]; 8],
}

impl HangmanCanvas {
    pub fn new() -> Result<HangmanCanvas, std::io::Error> {
        let mut file = File::open("src/hangman.ascii")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut grid = [[' '; 14] ; 8];
        for (x, line) in contents.lines().enumerate() {
            for (y, letter) in line.chars().enumerate() {
                grid[x][y] = letter;
            }
        }

        Ok(HangmanCanvas { grid })
    }

    pub fn print(&self) {
        for line in self.grid.iter() {
            for character in line.iter() {
                print!("{}", character);
            }
            println!();
        }
    }
}

fn main() -> std::io::Result<()> {
    let canvas = HangmanCanvas::new()?;
    canvas.print();

    Ok(())
}
