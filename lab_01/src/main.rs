mod distance;

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Helol");

    println!(
        "Distance between {} and {} is {}",
        s1,
        s2,
        distance::damerlau_levenstein_iter(&s1, &s2)
    );
}
