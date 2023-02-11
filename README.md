# fake-ua
<<<<<<< HEAD
`fake-ua`是一个基于 Rust 语言编写的库，用于生成伪造的 User-Agent 字符串。通过使用该库，您可以轻松地生成各种各样的 User-Agent 字符串，以便用于网络爬虫、测试等应用场景。`fake-ua`具有简单易用的API，可以生成符合标准的 User-Agent 字符串。

## 安装
在您的 Cargo.toml 文件中添加以下行：
```toml
[dependencies]
fake-ua = "0.1.0"
```

## 使用
生成 User-Agent
```rust
use fake_ua::UserAgent;

fn main() {
    let user_agent = UserAgent::new();
    let ua_str = user_agent.random();
    println!("{}", ua_str);
}
```
## 贡献
欢迎任何形式的贡献！如果您发现任何错误或有任何改进意见，请随时向我们发送请求。
=======
fake-ua是一个基于Rust语言编写的库，用于生成伪造的User-Agent字符串。通过使用该库，您可以轻松地生成各种各样的User-Agent字符串，以便用于网络爬虫、测试等应用场景。fake-ua具有简单易用的API，支持自定义User-Agent模板和随机化参数，可以生成符合标准的User-Agent字符串。
>>>>>>> ab226a1a2198dfe127d3d7a00c234a7c670ef6c3
