use std::io;
use rand::prelude::*;

fn main() {
    let guess_list = ["apple", "banana", "grapes", "pineapple"];

    let mut rng = thread_rng();
    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];

   loop{
    let mut input = String::new();

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            let fruit = input.trim().to_lowercase();

            if guess_checker(&fruit,random_fruit){
                println!("You guessed it!");
                break;
            }else{
                println!("Try again!");
                println!("{}",random_fruit)
            }

        }
        Err(error)=>{
            println!("Error: {}", error);
        }

    }
   }

}
fn guess_checker(guess: &str, target: &str) -> bool {
    if guess == target {
        return true;
    }else{
        return false;
    }
}
