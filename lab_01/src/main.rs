mod distance;
use std::io;

fn main() -> io::Result<()> {
    let mut s1 = String::new();
    let mut s2 = String::new();

    println!("Input 2 strings:");

    io::stdin().read_line(&mut s1)?;
    io::stdin().read_line(&mut s2)?;

    let s1 = s1.trim_end();
    let s2 = s2.trim_end();

    println!("Levenstein rec: {}", distance::levenstein_rec(s1, s2));
    println!("Levenstein rec memoized: {}", distance::levenstein_mem_rec(s1, s2));
    println!("Levenstein iter: {}", distance::levenstein_iter(s1, s2));
    println!("damerau-Levenstein rec: {}", distance::damerau_levenstein_rec(s1, s2));
    println!("damerau-Levenstein iter: {}", distance::damerau_levenstein_iter(s1, s2));

    Ok(())
}
