// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM   DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    // 定义带不同数据类型的枚举变体，匹配 main 中的使用场景
    Quit,                          // 无数据变体
    Echo(String),                  // 携带 String 类型数据
    Move { x: i32, y: i32 },       // 结构体样式的变体，携带 x/y 两个 i32 字段
    ChangeColor(u8, u8, u8),       //
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
