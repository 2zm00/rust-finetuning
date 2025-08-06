use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
}

async fn list_todos() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Todo { id: 1, title: "첫번째 할 일".into() },
        Todo { id: 2, title: "두번째 할 일".into() },
    ])
}

async fn create_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(create_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
