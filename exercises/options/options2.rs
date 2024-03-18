// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";//字符串引用
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {//rust中..是半开区间，表示包括前面不包括后面
            optional_integers.push(Some(i));
        }

        let mut cursor = range;//整数复制，所有权没转移，执行了复制。

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // integer = optional_integers.pop() {//optional_integers.pop()返回的是Option<Option<i8>>
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }
        while let Some(Some(integer)) = optional_integers.pop() {//optional_integers.pop()返回的是Option<Option<i8>>，需要定义的类型相匹配
            assert_eq!(integer, cursor);
            cursor -= 1;
        }


        assert_eq!(cursor, 0);
    }
}
