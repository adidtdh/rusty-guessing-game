use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Guess my number!");

    let secret_number = rand::thread_rng().gen_range(0, 10);

    loop{

        println!("Enter a guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        {

            let guess = guess.trim().parse::<i32>().expect("Failed to read integer");


            match guess.cmp(&secret_number){
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too Big"),
                Ordering::Equal =>{
                    println!("You Win");
                    break
                } 
            }
        }
    }


}
