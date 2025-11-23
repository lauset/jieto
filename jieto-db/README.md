# jieto-db

# 配置文件(db.toml)
```toml
[[sqlite]]
name = "sqlite"
url = "******"


[[mysql]]
default = true
name = "mysql"
host = "******"
port = 3306
username = "******"
password = "******"
database = "******"


[[mysql]]
name = "mysql1"
host = "******"
port = 3306
username = "******"
password = "******"
database = "******"


[[postgres]]
name = "postgres"
host = "******"
port = 15400
username = "******"
password = "******"
database = "******"
schema = "******"

[[redis]]
name = "redis"
host = "******"
port = 6379
password = "******"
db = 1

```

## 使用方法
```rust
let db_manager = jieto_db::jieto_db_init("db.toml").await?;
let pool = db_manager.with_mysql_default()?;
```
