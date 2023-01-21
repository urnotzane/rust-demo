fn main() {
    multiple_if();
    let_if();
    loop_fn();
    loop_label();
    while_fn();
    for_fn();
}
fn for_fn() {
    let arr = [1,2,3,4,5];
    for element in arr {
        println!("{element}")
    }
}
fn while_fn() {
    let mut count = 0;
    while count < 5 {
        count += 1;
        println!("循环中...{count}");
    }
    println!("循环结束：{count}")
}
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // 打断counting_up循环
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("counting_up循环结束，count = {count}");
}
fn loop_fn() {
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // 通过分号结束将结果赋值给变量`result`
    
    println!("counter：{result}"); // counter：20
}
fn let_if() {
    let num = if true { 5 } else { 9 };

    println!("{num}")
}
fn multiple_if() {
    let num = 5;

    if num < 5 {
        println!("num小于5");
    } else if num > 5 {
        println!("num大于5")
    } else {
        println!("num等于5")
    }
}
