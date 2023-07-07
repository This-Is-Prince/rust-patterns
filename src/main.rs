fn main() {
    println!("---- Star Patterns ----");
    first();
    second();
    third();
    fourth();
    fifth();
}

fn first() {
    println!("1. First Pattern\n");
    let row = 5;
    let col = 5;
    for _ in 0..row {
        for _ in 0..col {
            print!("*");
        }
        println!();
    }
    println!("\n")
}

fn second() {
    println!("2. Second Pattern\n");
    let row = 5;
    let col = 5;
    for i in 0..row {
        for j in 0..col {
            if j <= i {
                print!("*")
            }
        }
        println!();
    }
    println!("\n")
}

fn third() {
    println!("3. Third Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        for j in 1..=col {
            if j <= i {
                print!("{j}")
            }
        }
        println!();
    }
    println!();
}

fn fourth() {
    println!("4. Fourth Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        for j in 1..=col {
            if j <= i {
                print!("{i}")
            }
        }
        println!();
    }
    println!();
}

fn fifth() {
    println!("5. Fifth Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        for j in 1..=col {
            if j <= col + 1 - i {
                print!("*")
            }
        }
        println!();
    }
    println!();
}