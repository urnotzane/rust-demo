// 声明函数
fn main() {
    println!("Hello, world!");
    // 调用函数
    another_function(5, 'z');

    let x = five();
    // 输出返回值
    println!("five()：{x}")
}
// 声明函数
fn another_function(num: i32, unit_label: char) {
    println!("another_function：{num}、{unit_label}");
}

fn five() -> i32 {
    5
}