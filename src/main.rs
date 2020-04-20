mod hcanvas;

struct GameState {
    goal: String,
    display: Vec<char>,
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

        let state = GameState {
            goal: goal.to_lowercase().to_string(),
            display: vec!['_'; goal.len()],
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
            if self.has_letter(c) {
                self.handle_hit(c);
            }
            else {
                self.handle_miss();
            }
        }
        else {
            println!("you already guessed {}!", lower);
            println!("guessed letters: {:?}", self.state.guesses);
        }
    }

    pub fn show(&self) {
        self.canvas.print_step(self.state.misses);
        self.display_current();
    }

    fn has_letter(&self, c: char) -> bool {
        return self.state.goal.contains(c);
    }

    fn display_current(&self) {
        println!("{}", self.state.display.iter().map(|c| c.to_string() + " ").collect::<String>());
        if self.state.playing {
            println!("Guess a letter:");
        }
    }

    fn handle_miss(&mut self) {
        self.state.misses += 1;
        println!("oh no, you lost your {}!", hcanvas::REVEAL_STR[self.state.misses - 1]);
        if self.state.misses == hcanvas::STEPS {
            self.state.playing = false;
        }

        self.show();

        if !self.state.playing {
            println!("YOU LOSE! :(");
            println!("the word was '{}'", self.state.goal);
        }
    }

    fn handle_hit(&mut self, c: char) {
        println!("nice job!");

        for (i, letter) in self.state.goal.chars().enumerate() {
            if letter == c {
                self.state.display[i] = c;
            }
        }

        if self.state.display.iter().collect::<String>() == self.state.goal {
            self.state.playing = false;
        }

        self.show();

        if !self.state.playing {
            println!("YOU WIN!!");
        }
    }
}


fn main() -> std::io::Result<()> {
    let mut hangman = Hangman::new("platypus")?;

    println!("Welcome to Rusty Hangman!");
    hangman.show();

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
