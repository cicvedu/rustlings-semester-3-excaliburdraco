// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());//result生命周期与string1和string2中短的一致。
        // println!("The longest string is '{}'", result);
    }
    
    /*注意，result 必须在 string2 的生命周期结束之前使用，否则会出现编译错误，
    因为 result 的生命周期与 string1 和 string2 中较短的那个相同。在这个例子中，string2 的生命周期比
    string1 短，因为它在内部作用域中创建，并在该作用域结束时被丢弃。
    因此，result 的生命周期与 string2 相同，并且在 string2 被丢弃后不能再使用 result。
    这就是为什么我们在 string2 的作用域内打印 result。这样可以确保在我们使用 result 时，
    它引用的数据仍然有效。如果我们试图在 string2 的作用域之外打印 result，我们将得到一个编译错误，
    因为 result 的生命周期已经结束。这就是Rust的所有权和生命周期系统如何帮助我们避免数据竞争和悬挂引用的。
    希望这个解释对你有所帮助！ */
}
