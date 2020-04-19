mod hcanvas;

fn main() -> std::io::Result<()> {
    let hangman = hcanvas::HangmanCanvas::new()?;

    let one_sec = std::time::Duration::from_millis(1000);

    let mut n = 0;
    while n <= hcanvas::STEPS {
        println!("STEP {} of game:", n);
        hangman.loss(n);

        std::thread::sleep(one_sec);
        n += 1;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read");

    Ok(())
}
