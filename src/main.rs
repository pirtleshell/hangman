
struct HangmanCanvas {
    grid: [[char; 14]; 8],
}

impl HangmanCanvas {
    pub fn new() -> Result<HangmanCanvas, std::io::Error> {
        let contents = std::fs::read_to_string("src/hangman.ascii")
            .expect("Can't find hangman!");

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
