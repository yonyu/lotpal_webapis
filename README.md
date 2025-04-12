# LotPal Application Backend

```
cargo new lotpal_webapis

git branch -M master
git remote add origin https://github.com/yonyu/lotpal_webapis.git
git push -u origin master


Add sqlite executable to PATH: 'C:\DevTools\sqlite'
Add environment variable SQLITE3_LIB_DIR = C:\DevTools\vcpkg\installed\x64-windows-static\lib

run command to create dukair.db from assets/database.sql
sqlite3.exe dukair.db ".read database.sql"


rusqlite = { version = "0.34.0", features = ["bundled"] }
"bundled" means it locates the library file from SQLITE3_LIB_DIR

```Cargo.toml
[dependencies]
actix = "0.13.5"
actix-web = "4"
config = "0.15.11"
env_logger = "0.11.8"
rusqlite = { version = "0.34.0", features = ["bundled"] }
serde = {version = "1.0.219", features = ["derive"]}
serde_json = "1.0.140"
uuid = {version = "1.16.0", features = [ "v4", "v7", ]}
```


#![allow(unused)]

http://127.0.0.1:3000/api/v1/flightplan


