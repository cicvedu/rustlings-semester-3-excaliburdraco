// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    /*这个错误是因为你的宏 my_macro! 的定义中，
    两个模式没有被正确地分隔开。在 Rust 中，你需要使用 ;
     来分隔宏定义中的不同模式1234。 */
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
