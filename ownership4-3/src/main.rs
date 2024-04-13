fn main() {
    let s = String::from("hello");  // s 进入作用域
    
    takes_ownership(s);  // s 的值移动到函数里面，所以之后不再有效

    let x = 5;             // x 进入作用域

    makes_copy(x);  // x应该移动到函数里，但 i32 是 Copy 的，所以在后面可继续使用 x

}

fn takes_ownership(some_string: String) {   // some_string 进入作用域
    println!("{}", some_string);
}   // 这里， some_string 离开作用域并调用 `drop`，释放内存

fn makes_copy(some_integer: i32) {   // some_integer 进入作用域
    println!("{}", some_integer);
}   // 这里， some_integer 离开作用域。不会有特殊操作