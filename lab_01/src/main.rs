mod distance;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // let s1 = String::from("testtesttesttest");
    // let s2 = String::from("restrestrestrest");

    let mut s1 = String::new();
    let mut s2 = String::new();

    println!("Input 2 strings:");

    io::stdin().read_line(&mut s1)?;
    io::stdin().read_line(&mut s2)?;

    let s1 = s1.trim_end();
    let s2 = s2.trim_end();

    println!("Levenstein rec: {}", distance::levenstein_rec(s1, s2));
    println!("Levenstein iter: {}", distance::levenstein_iter(s1, s2));
    println!("Damerlau-Levenstein rec: {}", distance::damerlau_levenstein_rec(s1, s2));
    println!("Damerlau-Levenstein iter: {}", distance::damerlau_levenstein_iter(s1, s2));

    Ok(())
}
