// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    // 核心实现：接收 self（转移所有权），追加 "Bar" 后返回
    fn append_bar(self) -> Self {
        // 1. 将 self 转为可变字符串（因要修改）
        let mut s = self;
        // 2. 追加 "Bar"（push_str 是高效的字符串追加方法）
        s.push_str("Bar");
        // 3. 返回修改后的字符串（符合 Self 类型要求）
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
