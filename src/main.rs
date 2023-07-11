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
    eleventh();
    twelfth();
    thirteen();
    fourteen();
    fifteen();
}

fn fifteen() {
    println!("15. Fifteen Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        let mut result = 65;
        for j in 1..=col {
            if j <= col + 1 -i {
                print!("{:}", std::char::from_u32(result).unwrap());
                result += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("\n");
}

fn fourteen() {
    println!("14. Fourteen Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        let mut result = 65;
        for j in 1..=col {
            if j <= i {
                print!("{:}", std::char::from_u32(result).unwrap());
                result += 1;
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("\n");
}

fn thirteen() {
    println!("13. Thirteen Pattern\n");
    let row = 5;
    let col = 5;
    let mut result = 1;
    for i in 1..=row {
        for j in 1..=col {
            if j <= i {
                print!("{result}");
                result += 1;
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("\n");
}

fn twelfth() {
    println!("12. Twelfth Pattern\n");
    let row = 4;
    let col = 8;
    for i in 1..=row {
        for j in 1..=col {
            if j <= i || j >= col + 1 - i {
                if j <= i {
                    print!("{j}");
                } else {
                    let result = col + 1 - j;
                    print!("{result}");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("\n");
}

fn eleventh() {
    println!("11. Eleventh Pattern\n");
    let row = 5;
    let col = 5;
    for i in 1..=row {
        let mut print = i % 2;
        for j in 1..=col {
            if j <= i {
                print!("{print}");
                if print == 1 {
                    print = 0;
                } else {
                    print = 1;
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("\n");
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