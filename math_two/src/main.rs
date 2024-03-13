use std::io;

fn get_num() -> i32 {
    let num: i32 = loop {
        println!("Please input a number:");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Could not read input");

        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hey! That's not a number!");
                continue;
            }
        };
        break num;
    };
    num
}

fn main() {
    let n1 = get_num();
    let n2 = get_num();

    let mut op = String::new();
    let res = loop {
        println!("Please input an operator(+,-,*,/)");
        op.clear();

        io::stdin()
            .read_line(&mut op)
            .expect("Could not read input.");

        let res = match op.trim() {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            "/" => n1 / n2,
            _ => {
                println!("Not an Operator!");
                continue;
            }
        };
        break res;
    };

    println!("{n1} {} {n2} = {res}", op.trim())
}
