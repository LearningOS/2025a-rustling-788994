// enums1.rs
//
// No hints this time! ;)

// I AM   DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    // 定义枚举变体，匹配 main 中使用的所有类型
    Quit,
    Echo,
    Move,
    ChangeColor,
}




fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
