
pub const STEPS: usize = 6;
const COLS: usize = 14;
const ROWS: usize = 8;
const EMPTY: char = ' ';
pub const REVEAL_STR: [&str; STEPS] = ["head", "body", "left arm", "right arm", "left leg", "right leg"];
const REVEAL_SEQ: [char; STEPS] = ['H', 'B', 'L', 'R', 'l', 'r'];

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct HangmanCanvas {
    /** the grid of characters at step 0 of the game. ie. the gallows */
    grid: [[char; COLS]; ROWS],

    /** the characters that get revealed, ie. the hangman */
    reveal_grid: [[char; COLS]; ROWS],

    /** a vector of the Points to reveal at each step of the game. */
    reveals: [Vec<Point>; STEPS],
}

impl HangmanCanvas {
    /** creates a new HangmanCanvas. initializes the grid and reveal data from ascii files */
    pub fn new() -> Result<HangmanCanvas, std::io::Error> {
        let contents = std::fs::read_to_string("src/hangman.ascii")
            .expect("Can't find hangman!");

        let mut grid = [[EMPTY; COLS]; ROWS];
        for (x, line) in contents.lines().enumerate() {
            for (y, letter) in line.chars().enumerate() {
                grid[x][y] = letter;
            }
        }

        let mask_contents = std::fs::read_to_string("src/hangman_mask.ascii")
            .expect("Can't find hangman mask!");

        // the following only works because STEPS <= 32
        let mut reveals: [Vec<Point>; STEPS] = Default::default();
        let mut reveal_grid = [[EMPTY; COLS]; ROWS];

        for (x, line) in mask_contents.lines().enumerate() {
            for (y, letter) in line.chars().enumerate() {
                if letter != EMPTY {
                    let index = REVEAL_SEQ.iter().position(|&r| r == letter)
                        .expect(&format!("Unrecognized character '{}' in mask!", letter));

                    let point = Point { x, y };
                    reveals[index].push(point);

                    reveal_grid[x][y] = grid[x][y];
                    grid[x][y] = EMPTY;
                }
            }
        }

        Ok(HangmanCanvas { grid, reveal_grid, reveals })
    }

    /** print the grid at the nth step of the game */
    pub fn print_step(&self, step: usize) {
        let index = if step > STEPS { STEPS } else { step };
        let excludes = &self.reveals[0..index].iter().flatten().collect::<Vec<_>>();
        let mut draw = self.grid.clone();

        for point in excludes.iter() {
            draw[point.x][point.y] = self.reveal_grid[point.x][point.y];
        }

        HangmanCanvas::print_grid(draw);
    }

    /** print an entire grid array */
    fn print_grid(grid: [[char; COLS]; ROWS]) {
        for line in grid.iter() {
            for character in line.iter() {
                print!("{}", character);
            }
            println!();
        }
    }
}
