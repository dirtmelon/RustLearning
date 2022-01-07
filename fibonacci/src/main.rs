use std::io;

fn main() {
    loop {
        let mut choice = String::new();
        println!("1. 请输入斐波那契数列的阶数");
        println!("2. 退出");
        io::stdin().read_line(&mut choice).expect("读取失败");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => {
                println!("请输入阶数: ");
                let mut n = String::new();
                io::stdin().read_line(&mut n).expect("读取失败");
                let n: i32 = match n.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let result = fibonacci(n);
                println!("{} 的斐波那契数列为 {}", n, result);
            }
            2 => {
                println!("退出");
                break;
            }
            _ => {
                println!("输入错误");
                continue;
            }
        }
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut first: i32 = 0;
    let mut second: i32 = 1;
    let mut n = n;
    while n > 1 {
        let temp = first + second;
        first = second;
        second = temp;
        n -= 1;
    }
    return second;
}
