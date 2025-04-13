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

query:

sqlite3 assets/duckair.db
select * from users;

rusqlite = { version = "0.34.0", features = ["bundled"] }
"bundled" means it locates the library file from SQLITE3_LIB_DIR

```Cargo.toml
serde = {version = "1.0.219", features = ["derive"]}
serde_json = "1.0.140"
rusqlite = { version = "0.34.0", features = ["bundled"] }
uuid = {version = "1.16.0", features = [ "v4", "v7", ]}
config = "0.15.11"
env_logger = "0.11.8"
actix = "0.13.5"
actix-web = {version = "4.3.0", features= ["openssl"]}
actix-web-httpauth = "0.8.0"
openssl-sys = "0.9"
openssl = { version = "0.10.72", features = ["vendored"] }
actix-cors = "0.7.1"
rustls = "0.23.26"
```

## Build your Rust project which has openssl dependence on windows

Reference: https://docs.rs/openssl/latest/openssl/

1) Download and install openssl runtime full version
https://slproweb.com/products/Win32OpenSSL.html

2) Set environment variables and path:

$env:OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib\VC\x64\MT"
$env:OPENSSL_INCLUDE_DIR="C:\Program Files\OpenSSL-Win64\include"

$env:PATH += ";C:\Program Files\OpenSSL-Win64\bin"

3) Install Strawberry Perl (x64)
https://strawberryperl.com/



#![allow(unused)]

cargo run

run postman

http://127.0.0.1:3000/api/v1/flightplan

Get unauthorized

Inside Postman, Authorization | Bearer Token | Token : f90ed373612749eda337c9a4d537c247

http://127.0.0.1:3000/api/v1/flightplan

or

https://127.0.0.1:3001/api/v1/flightplan

will get list of the plans.

Note: f90ed373612749eda337c9a4d537c247 is the api key for the default user: John Smith