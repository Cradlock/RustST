mod collector;
mod common;

fn main() {
    
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Please provide at least one argument.");
        return;
    }

    let command: &String = &args[1]; 



}



