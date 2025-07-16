use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:limingyu666@192.168.88.100:5432/short_url")
        .await
        .expect("数据库连接失败")
}