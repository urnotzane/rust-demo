// 引入io输入/输出库
use std::io;
use rand::Rng;

// 主函数声明
fn main() {
    println!("猜数字！");
    
    // 随机一个1~100的数字
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("请输入你的猜测（1~100）：");

    // 声明一个保存用户猜测数字的变量，`mut`代表的是可变的变量
    // `String::new()`相当于JS里的`new String()`，生成一个新的String空实例
    let mut guess = String::new();
    
    // 终端输入句柄
    io::stdin()
        // `read_line`读取终端输入并追加至传入的可变变量中，不是覆盖
        .read_line(&mut guess)
        .expect("读取失败。");
    
    println!("你猜测是数字是：{guess}");
    println!("随机的数字为：{secret_number}，你猜对了吗？！");
}
