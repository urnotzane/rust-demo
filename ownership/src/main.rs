fn main() {
    // str进入作用域
    let str = String::from("hello");
    // str移动至log_it函数
    log_it(str);
    // str无效

    // num进入作用域
    let num = 5;
    // num被复制到log_num函数
    log_num(num);
    // num继续有效

    // 将函数返回值移动至str2
    let str2 = gives_ownership();

    // 进入作用域
    let str3 = String::from("str3");
    // 将str3移动至函数，函数又将其返回并转移给str4
    let str4 = takes_and_gives_back(str3);
}

fn log_it(s: String) {
    println!("log_it {s}");
}
fn log_num(n: i32) {
    println!("log_num {n}");
}
fn gives_ownership() -> String {
    let str = String::from("ownership");
    str
}
fn takes_and_gives_back(str:String) -> String {
    str
}