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
                continue
            },
        };
        break num;
    };
    num
}


fn main() {
    let n1 = get_num();
    let n2 = get_num();

    if n1 > n2 {
        for i in n2..n1 {
            println!("{i}")
        }
    } else if n2 > n1 {
        for i in n1..n2 {
            println!("{i}")
        }
    } else {
        println!("These numbers are the same!")
    }
}
