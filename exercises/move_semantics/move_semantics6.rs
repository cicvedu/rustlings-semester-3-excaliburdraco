// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great! ".to_string();

    println!("{}",get_char(&data));//字符串传递参数的时候，拷贝了指向堆内存的引用。
    //如果不用&符号，那么data的值会被拷贝，而get_char函数的参数是引用，所以会报错。

    string_uppercase(&mut data.clone());
    //data.clone（）拷贝的时候，复制的堆内存上的数据
    //string_uppercase函数的参数是引用，所以不会报错。
 
}

// Should not take ownership
fn get_char(data:&String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();

    println!("{}", data);
}
