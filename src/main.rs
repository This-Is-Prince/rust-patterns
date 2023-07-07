fn main() {
    println!("---- Star Patterns ----");
    first();
}

fn first() {
    let row = 5;
    let col = 5;
    for _ in 0..row {
        for _ in 0..col {
            print!("*");
        }
        println!();
    }
}