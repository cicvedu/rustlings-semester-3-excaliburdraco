// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // "blue"    ////blue存储在静态存储区，既不在栈上，也不在堆上，在程序的可执行运行区。
    String::from("blue")
    //return String::from("blue");方法一

    // "blue".to_string()
    // //return "blue".to_string();方法二 to_string与to_owner的区别

    // "blue".to_owned()
    //return "blue".to_owned();方法三 转变之后，有所有权。不同类型实现to_owen后，类型会不一样。

    // "blue".into()
    //return "blue".into();方法四
}
