use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Enter your guess: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess: u32 = input.trim().parse().unwrap();

        println!("You guessed: {}", guess);

        if guess == 42 {
            println!("Correct!");
            break;
        } else {
            println!("Wrong!");
        }

        println!("");
    }
}
