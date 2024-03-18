// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
//方法一：
fn compare_license_types(software:&dyn Licensed, software_two:&dyn Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
/*方法一在这里，我们使用了动态分发（&dyn Licensed），这意味着 software 和 software_two 可以是任何实现了 
Licensed trait 的类型，不论它们是否是相同的类型。这样，你就可以比较 SomeSoftware 实例和 OtherSoftware 
实例了。但是，你需要确保你的 Licensed trait 和它的方法是对象安全的，也就是说它们可以在动态分发的上下文中
使用。在这个例子中，Licensed trait 和它的 licensing_info 方法是对象安全的，因为它们没有任何不兼容的特性，
如静态方法或关联常量。所以，你可以安全地使用 &dyn Licensed。这是 Rust 中的一种常见模式，用于处理不同类型
的值，而这些值都实现了相同的接口（在这里是 Licensed trait）。 */
/*

//方法二：
fn compare_license_types<T: Licensed>(software: T, software_two: T) -> bool {
    software.licensing_info() == software_two.licensing_info()
} 
方法二在这里，T 是一个泛型参数，它代表了任何实现了 Licensed trait 的类型。这样，你就可以使用相同类型的
两个软件进行比较了。例如，你可以比较两个 SomeSoftware 实例，或者两个 OtherSoftware 实例。
但是，你不能将 SomeSoftware 实例和 OtherSoftware 实例进行比较，因为它们是不同的类型。
这是因为我们在函数签名中指定了 software 和 software_two 必须是相同的类型 T。
如果你想要比较不同类型的软件，你可以移除这个限制，像这样：
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(&some_software, &other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(&other_software, &some_software));
    }
}
