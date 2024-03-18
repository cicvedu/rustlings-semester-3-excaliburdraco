// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro");
        };
    }

// use crate::my_module::my_macro;
/*这个错误是因为你试图从 my_module 模块中导入 my_macro 宏，
但是 my_macro 宏并没有在 my_module 模块中被定义1。在你的代码中，
my_macro 是一个宏，而不是一个模块21。

在 Rust 中，使用 #[macro_export] 注解的宏会被导出到 crate 的根，
而不是它被定义的模块1。所以，你应该直接从 crate 的根导入 my_macro 宏，
而不是从 my_module 模块导入1。 */

// use crate::my_module::my_macro;

fn main() {
    my_macro!();
}