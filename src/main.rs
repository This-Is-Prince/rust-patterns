fn main() {
    println!("---- Star Patterns ----");
    first();
    second();
    third();
    fourth();
    fifth();
    sixth();
    seventh();
    eighth();
    ninth();
    tenth();
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

fn sixth() {
    println!("6. Sixth Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        for j in 1..=col {
            if j <= col + 1 - i {
                print!("{j}");
            }
        }
        println!();
    }
    println!();
}

fn seventh() {
    println!("7. Seventh Pattern\n");
    let row = 5;
    let col = 9;
    for i in 1..=row {
        for j in 1..=col {
            if j >= row + 1 - i && j <= row - 1 + i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn eighth() {
    println!("8. Eighth Pattern\n");
    let row = 5;
    let col = 9;
    for i in 1..=row {
        for j in 1..=col {
            if j >= i && j <= col + 1 - i {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn ninth() {
    println!("9. Ninth Pattern\n");
    let row = 10;
    let col = 9;
    for i in 1..=row {
        for j in 1..=col {
            if i <= row/2 {
                if j >= (row/2) + 1 - i && j <= (row/2) - 1 + i {
                    print!("*");
                } else {
                    print!(" ");
                }
            } else {
                if j >= i - (row/2) && j <= col + (row/2) + 1 - i {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
    println!();
}

fn tenth() {
    println!("10. Tenth Pattern\n");
    let row = 9;
    let col = 5;
    for i in 1..=row {
        for j in 1..=col {
            if i <= row/2 {
                if  j <= i {
                    print!("*");
                } else {
                    print!(" ");
                }
            } else {
                if j <= row + 1 - i {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
    println!();
}