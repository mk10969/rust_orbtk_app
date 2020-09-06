# rust_orbtk_app

http://diesel.rs/guides/getting-started/

### diesel cli install

cargo install diesel_cli --no-default-features --features postgres

### .env に、DB 接続先を追加

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

### diesel setup -> migration 用の function を作成する

diesel setup

### create table photos

diesel migration generate create_photos

### up.sql and down.sql に、SQL 文を書く

create table では、必ず PK の制約を入れた table にすること！

### apply migration table

diesel migration run

### rollback (=redoing) migration table

diesel migration redo


## 存在するtablekからtable!マクロとモデルを自動生成する方法

### if exist table (reverse engineering)

diesel print-schema　 > src/schema.rs

### reverse create model struct (before: cargo install cargo install diesel_cli_ext)

deisel_ext --model

