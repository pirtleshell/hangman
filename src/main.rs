mod hcanvas;

struct GameState {
    goal: String,
    guesses: Vec<char>,
    misses: usize,
    playing: bool,
}

struct Hangman {
    canvas: hcanvas::HangmanCanvas,
    state: GameState,
}

impl Hangman {
    pub fn new(goal: &str) -> Result<Hangman, std::io::Error> {
        let canvas = hcanvas::HangmanCanvas::new()?;

        let mut state = GameState {
            goal: goal.to_string(),
            guesses: Vec::new(),
            misses: 0,
            playing: true,
        };

        Ok(Hangman { canvas, state })
    }

    pub fn guess(&mut self, c: char) {
        let lower = c.to_lowercase().next().unwrap();
        if !self.state.guesses.contains(&lower) {
            println!("you guessed '{}'", lower);
            self.state.guesses.push(lower);
        }
        else {
            println!("you already guessed {}!", lower);
        }
        println!("guessed chars: {:?}", self.state.guesses);
    }

    pub fn show(&self) {
        self.canvas.print_step(self.state.misses);
    }
}


fn main() -> std::io::Result<()> {
    let mut hangman = Hangman::new("platypus")?;

    while hangman.state.playing {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("failed to read");

        // guess length should be 2 including newline
        if guess.len() == 1 {
            println!("enter a guess!");
        }
        else if guess.len() > 2 {
            println!("only one letter at a time!");
        }
        else {
            let c = guess.chars().next().unwrap();
            hangman.guess(c);
        }
    }

    Ok(())
}
