# Rust Enum Demo

## 简介

演示 Rust 枚举和模式匹配。

## 基本原理

枚举可以包含数据，是 Rust 实现类型安全的重要工具。

## 启动和使用

```bash
cargo run
```

## 教程

### 基础枚举

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

### 模式匹配

```rust
match msg {
    Message::Write(text) => println!("写入: {}", text),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    _ => println!("其他"),
}
```

### if let

```rust
if let Some(value) = some_option {
    println!("值是: {}", value);
}
```
