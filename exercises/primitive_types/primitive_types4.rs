// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


// my_hint: slice
// 切片（slice）类型和数组类似，但其大小在编译时是不确定的。
// 相反，切片是一个双字对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度。
// 这个 “字” 的宽度和 usize 相同，由处理器架构决定，比如在 x86-64 平台上就是 64 位。
// slice 可以用来借用数组的一部分。slice 的类型标记为 &[T], 数组的类型标记为 [T; length]。


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
