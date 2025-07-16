use serde::Serialize;

// #[derive(sqlx::FromRow, Debug)]
// pub struct Users {
//     pub id: i32,
// }

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ShortLinks {
    pub id: String,
    pub short_url: String,
    pub user_id: i32,
    pub original_url: String,
    pub code: Option<String>,
}
