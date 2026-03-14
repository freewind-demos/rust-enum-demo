fn main() {
    println!("=== Rust 枚举和模式匹配演示 ===\n");

    // 1. 基础枚举
    let ipv4 = IpAddr::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddr::V6(String::from("::1"));

    // 2. 枚举与模式匹配
    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: RGB({}, {}, {})", r, g, b),
    }

    // 3. if let 简化匹配
    let some_value = Some(5);
    if let Some(5) = some_value {
        println!("值是 5！");
    }

    // 4. 枚举作为状态机
    let coin = Coin::Quarter(UsState::California);
    let value = value_in_cents(&coin);
    println!("硬币价值: {} cents", value);

    // 5. 复杂的枚举用法
    let config = WebEvent::PageLoad;
    let result = process_event(&config);
    println!("事件处理结果: {}", result);

    println!("\n=== 总结 ===");
    println!("枚举可以包含数据，类型安全");
    println!("模式匹配 (match) 是 Rust 的核心控制结构");
    println!("if let 是 match 的简化形式");
    println!("枚举可以实现状态机模式");
}

// 基础枚举
enum IpAddr {
    V4(String),
    V6(String),
}

// 带命名字段的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 关联常量的枚举
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
    Paste(String),
}

fn process_event(event: &WebEvent) -> String {
    match event {
        WebEvent::PageLoad => "页面加载".to_string(),
        WebEvent::PageUnload => "页面卸载".to_string(),
        WebEvent::KeyPress(c) => format!("按键: {}", c),
        WebEvent::Click { x, y } => format!("点击: ({}, {})", x, y),
        WebEvent::Paste(s) => format!("粘贴: {}", s),
    }
}

// 枚举用于状态机
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // ... 其他州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州!", state);
            25
        }
    }
}
