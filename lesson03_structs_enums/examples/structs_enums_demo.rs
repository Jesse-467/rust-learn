//! 第 3 课随堂演示：struct / enum / match / Option / Result / ?
//! 运行方式：cargo run --example structs_enums_demo

#[derive(Debug, Clone, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    tls: bool,
}

impl ServerConfig {
    // 关联函数（构造函数），类似 Go 的 NewServerConfig
    fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            tls: false,
        }
    }

    // 不可变借用方法（只读）
    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    // 可变借用方法（原地修改）
    fn enable_tls(&mut self) {
        self.tls = true;
    }

    // 消费所有权的方法（常用于 builder 或类型转换）
    fn with_port(self, port: u16) -> Self {
        Self { port, ..self } // 结构体更新语法
    }
}

#[derive(Debug)]
enum AuthEvent {
    Login { user: String, ip: String },
    Logout(String),
    Heartbeat,
}

#[derive(Debug)]
enum ParseError {
    Empty,
    InvalidNumber(String),
}

fn parse_positive(s: &str) -> Result<u32, ParseError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    let n: u32 = trimmed
        .parse()
        .map_err(|_| ParseError::InvalidNumber(trimmed.to_string()))?;
    if n == 0 {
        return Err(ParseError::InvalidNumber("0".to_string()));
    }
    Ok(n)
}

fn double_parsed(s: &str) -> Result<u32, ParseError> {
    // `?` 会在遇到 Err 时直接提前返回该 Err，遇到 Ok(v) 时解包出 v
    let n = parse_positive(s)?;
    Ok(n * 2)
}

fn main() {
    println!("=== 1. struct & impl ===");
    let mut cfg = ServerConfig::new("127.0.0.1", 8080);
    cfg.enable_tls();
    let cfg2 = cfg.with_port(9090); // cfg 已被 move
    println!("cfg2 = {cfg2:?}, addr = {}", cfg2.addr());

    println!("\n=== 2. enum & match / if let ===");
    let events = [
        AuthEvent::Login {
            user: "alice".to_string(),
            ip: "10.0.0.1".to_string(),
        },
        AuthEvent::Heartbeat,
        AuthEvent::Logout("alice".to_string()),
    ];
    for ev in &events {
        match ev {
            AuthEvent::Login { user, ip } => println!("登录: {user} @ {ip}"),
            AuthEvent::Logout(user) => println!("登出: {user}"),
            AuthEvent::Heartbeat => println!("心跳"),
        }
    }
    if let AuthEvent::Logout(u) = &events[2] {
        println!("if let 精准匹配到登出用户: {u}");
    }

    println!("\n=== 3. Option<T> ===");
    let nums = [10, 20, 30];
    let second = nums.get(1).copied().unwrap_or(0);
    let missing = nums.get(99).copied().unwrap_or(-1);
    println!("second = {second}, missing = {missing}");

    println!("\n=== 4. Result<T, E> & ? ===");
    for input in [" 21 ", "", "abc", "0"] {
        println!("double_parsed({input:?}) = {:?}", double_parsed(input));
    }
}
