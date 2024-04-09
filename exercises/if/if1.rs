// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.
// my_hint: if 
// if 语法并不像其它语言是“语句（statement）”，而是一个“表达式（expression）
// 可以直接将 if 表达式赋值给变量
// let number = if condition { 5 } else { 6 };

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {a} else {b}
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
