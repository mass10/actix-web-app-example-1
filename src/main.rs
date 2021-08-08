//!
//!
//!

use actix_web::{get, web, App, HttpServer, Responder};

mod application;
mod db;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

// #[get("/dashboard/")]
// async fn index(web::Path()) -> impl Responder {
//     format!("dashboard:")
// }

#[allow(unused)]
fn test1() {
    // let app = App::new("test1");
}

#[actix_web::main]
async fn main() {
    // アプリケーションを作成
    let mut application = application::Application::new();
    let result = application.run().await;
    if result.is_err() {
        let err = result.err().unwrap();
        println!("[ERROR] {}", err);
        return;
    }
    // if false {
    //     let _result = HttpServer::new(|| App::new().service(index))
    //         .bind("127.0.0.1:8080")?
    //         .run()
    //         .await
    //     return _result;
    // }
}
