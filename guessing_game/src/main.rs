// 引入io输入/输出库
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 主函数声明
fn main() {
    println!("猜数字！");

    // 随机一个1~100的数字
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // 声明一个保存用户猜测数字的变量，`mut`代表的是可变的变量
        // `String::new()`相当于JS里的`new String()`，生成一个新的String空实例
        let mut guess = String::new();

        println!("请输入你的猜测（1~100）：");

        // 终端输入句柄
        io::stdin()
            // `read_line`读取终端输入并追加至传入的可变变量中，不是覆盖
            .read_line(&mut guess)
            .expect("读取失败。");

        // 转换输入的字符为数字，如果转换失败，继续循环而不是退出程序
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
              println!("请输入数字！");
              continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                // 停止循环
                break;
            }
        }
    }
}
