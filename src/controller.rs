use crate::model::ShortLinks;
use crate::utils::generate_code;
use actix_web::http::header;
use actix_web::{HttpResponse, Responder, delete, get, post, web};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ShortenReq {
    // 用户标识直接接口传吧
    pub user_id: i32,
    pub original_url: String,
    pub custom_code: Option<String>,
}

/**
短连接生成接口
*/
#[post("/shorten")]
async fn shorten(req: web::Json<ShortenReq>, pool: web::Data<PgPool>) -> impl Responder {
    // let list: Vec<Users> = sqlx::query_as("SELECT id FROM users")
    //     .fetch_all(pool.get_ref())
    //     .await
    //     .unwrap();
    // println!("数据库中的所有数据：{:#?}", list);

    let user_exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(req.user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap();
    if !user_exists.0 {
        return HttpResponse::BadRequest().body(format!("用户{}不存在", req.user_id));
    }

    let code = req.custom_code.clone().unwrap_or_else(generate_code);
    let short_url = format!("http://127.0.0.1:8080/short-url/{}", code);

    let code_used: (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM short_links WHERE code = $1)")
            .bind(&code)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();
    if code_used.0 {
        return if (&req).custom_code.is_some() {
            HttpResponse::Conflict().body(format!(
                "当前指定编码“{}”已被使用",
                req.custom_code.clone().unwrap()
            ))
        } else {
            HttpResponse::Conflict().body("随机编码生成异常")
        };
    }

    let short_links_id = Uuid::new_v4().to_string();
    let url = if req.original_url.starts_with("http://") || req.original_url.starts_with("https://")
    {
        req.original_url.clone()
    } else {
        format!("http://{}", req.original_url)
    };
    sqlx::query!(
        "INSERT INTO short_links (id, code, short_url, original_url, user_id) VALUES ($1, $2, $3, $4, $5)",
        short_links_id, code, short_url, url, req.user_id
    )
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({ "short_url": short_url }))

    // HttpResponse::Ok().body(req)
}

/**
短链接重定向
*/
#[get("/short-url/{code}")]
pub async fn redirect(code: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "SELECT original_url FROM short_links WHERE code = $1",
        code.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    if let Some(row) = result {
        HttpResponse::Found()
            .insert_header((header::LOCATION, row.original_url))
            .finish()
    } else {
        HttpResponse::NotFound().body("链接错误，无法重定向")
    }
}

/**
管理：查询
*/
#[get("/list-links")]
pub async fn list_links(
    user: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = match user.get("user_id").and_then(|uid| uid.parse::<i32>().ok()) {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("用户ID错误"),
    };

    let links: Vec<ShortLinks> = sqlx::query_as(
        "SELECT id, user_id, code, original_url, short_url FROM short_links WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(links)
}

/**
管理：删除
*/
#[delete("/delete/{id}")]
pub async fn delete_link(
    id: web::Path<String>,
    user: web::Query<std::collections::HashMap<String, String>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_id = match user.get("user_id").and_then(|uid| uid.parse::<i32>().ok()) {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("用户ID错误"),
    };
    let short_link_id = match Uuid::parse_str(id.as_str()).ok() {
        Some(u) => u,
        None => return HttpResponse::BadRequest().body("链接ID错误"),
    };

    let result = sqlx::query!(
        "DELETE FROM short_links WHERE id = $1 AND user_id = $2",
        short_link_id.to_string(),
        user_id
    )
    .execute(pool.get_ref())
    .await
    .unwrap();

    if result.rows_affected() > 0 {
        HttpResponse::Ok().body("删除成功")
    } else {
        HttpResponse::NotFound().body("本用户链接不存在")
    }
}
