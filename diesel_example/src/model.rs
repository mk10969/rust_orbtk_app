use chrono::NaiveDateTime;

// DBからのデータ取得用構造体
#[derive(Eq, PartialEq, Debug, Queryable)]
pub struct Photo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// // データ挿入用構造体
// #[derive(Insertable)]
// #[table_name = "photos"]
// pub struct PhotoNewForm<'a> {
//     pub name: &'a str,
//     pub description: Option<&'a str>,
// }

// // データ更新用構造体
// #[derive(AsChangeset)]
// #[table_name = "photos"]
// pub struct PhotoUpdateForm<'a> {
//     pub name: Option<&'a str>,
//     pub description: Option<&'a str>,
// }
