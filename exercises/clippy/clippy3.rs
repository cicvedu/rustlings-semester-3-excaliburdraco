// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.




#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some(){
        // my_option.unwrap();
        println!("This is some option!");
    }else{
        println!("Nothing to see here!");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    /*resize 方法用于改变向量的大小，并用指定的值填充新的空间。
    但是，resize 方法没有返回值，因此您不能在其后直接调用 clear 方法。 */
    my_empty_vec.clear();
    // my_empty_vec.shrink_to_fit();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
