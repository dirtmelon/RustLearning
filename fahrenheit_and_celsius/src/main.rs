use std::io;

fn main() {
    loop {
        println!("请选择输入华氏度或者摄氏度：");
        println!("1.华氏度");
        println!("2.摄氏度");
        println!("3.退出");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取失败");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => {
                println!("请输入华氏度: ");
                let mut fahrenheit = String::new();
                io::stdin().read_line(&mut fahrenheit).expect("读取失败");
                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("{} 华氏度 = {} 摄氏度", fahrenheit, celsius);
            }
            2 => {
                println!("请输入摄氏度: ");
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("读取失败");
                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
                println!("{} 摄氏度 = {} 华氏度", celsius, fahrenheit);
            }
            3 => {
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
