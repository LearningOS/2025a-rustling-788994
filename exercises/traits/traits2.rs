// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM NOT

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // 1. 将接收的 self（Vec<String> 所有权）转为可变绑定（才能修改向量）
        let mut vec = self;
        // 2. 向向量尾部追加 "Bar" 字符串（符合 "append Bar" 的语义）
        vec.push(String::from("Bar"));
        // 3. 返回修改后的向量（匹配 Self 类型要求）
        vec
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
