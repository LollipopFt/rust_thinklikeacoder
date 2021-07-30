use rand::Rng;
use std::time::Instant;

fn main() {
    for _ in 0..2 {
        let pass = pass_generation();
        println!("{:?}\n", pass);
        solver(pass);
    }
}

#[derive(Debug)]
struct CompPass {
    password: u8,
    colour: String,
}

fn pass_generation() -> CompPass {
    let password: u8 = rand::thread_rng().gen_range(0..101);
    println!("{}", password);
    CompPass {
        password,
        colour: "green".to_string(),
    }
}

fn solver(compass: CompPass) {
    println!("Solving...");
    let mut password: u8 = 0;
    let pass: u8 = compass.password;
    let colour: String = compass.colour;
    let start = Instant::now();
    loop {
        if password == pass {
            if colour == *"green" {
                break;
            }
        } else {
            password += 1
        }
    }
    let timetaken = start.elapsed();
    println!("Password: {}, Colour: {}", password, colour);
    println!("Time taken: {:?}\n", timetaken);
}
