use std::io;

fn main() {
    let mut x:String = String::new();
    let mut y:String = String::new();

    println!("Please input x: ");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    println!("Please input y: ");
    io::stdin()
        .read_line(&mut y)
        .expect("Falied to read line.");

    let mut x:i64 = x.trim().parse().expect("Please input a number.");
    let mut y:i64 = y.trim().parse().expect("Please input a number.");

    prepare(&mut x, &mut y);

    if x % y == 0 {
        println!("common divisor calculate.");
        println!("common divisor is {}", y);
    } else {
        loop_calculate(&x, &y);
    }

}

fn loop_calculate(first_number: &i64, second_number: &i64) {
    let residue:i64 = first_number - (2 * second_number);
    if second_number % residue == 0 {
        println!("{}", &residue);
    } else {
        loop_calculate(&second_number, &residue);
    }
}

fn log(sentence:&str) {
    println!("[INFO]: {}", sentence);

}

fn changeposition(first: &mut i64, second:&mut i64) {
    let t: i64;
    t = *first;
    *first = *second;
    *second = t;

}

fn prepare(first_num: &mut i64, second_num: &mut i64) {
    if first_num >= &mut (*second_num * 2) {
        log("success.")
    } else {
        log("First number is less than second number.");
        log("Change the position of two numbers.");
        changeposition(&mut *first_num, &mut *second_num);

        if second_num >= &mut (*first_num * 2) {
            log("mate success.");
        } else {
            println!("First number is less than second number, second number is also less than first number.")
        }
    }
}