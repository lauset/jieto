# jieto-db

## 配置文件 （application.toml）
```rust
name="test"
[web]
port = 9903

[log]
directory = "logs"
filename_prefix = "log"
max_size_mb = 100
age = "hour"
keep_files = 100
level = "info"
```

## 使用方法
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    jieto_web_start("application.toml", |cfg| {
        cfg.service(hello);
    })
    .await?;
    Ok(())
}
```