use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret = rand::thread_rng().gen_range(1,101);
    
    /*
    println!("the secret is: {}", secret);
    println!("You got: {}", guess); */

    loop{
		print!("Enter sum text: ");
		io::Write::flush(&mut io::stdout()).expect("Flush failed");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
		.expect("failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("You guessed : {}", guess);

		match guess.cmp(&secret) {
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
    }
}
