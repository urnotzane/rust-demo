fn main() {
    // 函数与所有权
    fn_ownership();
    // 返回值与作用域
    cb_ownership();
    // 引用
    reference();
    // 可变引用
    mutable_reference();
}

fn fn_ownership() {
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
}
fn log_it(s: String) {
    println!("log_it {s}");
}
fn log_num(n: i32) {
    println!("log_num {n}");
}
fn cb_ownership() {
    // 将函数返回值移动至str2
    let str2 = gives_ownership();

    // 进入作用域
    let str3 = String::from("str3");
    // 将str3移动至函数，函数又将其返回并转移给str4
    let str4 = takes_and_gives_back(str3);

    println!("str2 = {str2}, str4 = {str4}");
}
fn gives_ownership() -> String {
    let str = String::from("ownership");
    str
}
fn takes_and_gives_back(str: String) -> String {
    str
}
fn reference() {
    // 声明s1
    let s1 = String::from("hello");
    // 将指向`s1`的引用传递给函数
    let len = calculate_length(&s1);
    // 引用成功，s1继续有效
    println!("'{}'的长度是：{}", s1, len);
}
// 参数是个引用值
fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开了它的作用域

fn mutable_reference() {
    // 使用mut声明s1
    let mut s1 = String::from("hello");
    // 修改引用值
    change_reference(&mut s1);
    // 修改生效
    println!("s1的值：{}", s1);
}
// 参数定义为可变引用
fn change_reference(s: &mut String) {
    s.push_str(", world");
}