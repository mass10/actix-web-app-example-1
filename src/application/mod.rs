//!
//! アプリケーションのコア実装です。
//!

pub struct Application;

use super::db;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
	format!("Hello {}! id:{}", name, id)
}

// curl http://127.0.0.1:8080/1/masaru/index.html
impl Application {
	/// `Application` の新しインスタンスを作成します。
	///
	/// ## Returns
	// `Application` の新しいインスタンス
	pub fn new() -> Application {
		return Application {};
	}

	/// ウェブサーバーを起動します。
	pub async fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// データベース初期化
		db::init()?;

		// ウェブサーバーを起動
		let _context = HttpServer::new(|| App::new().service(index))
			.bind("127.0.0.1:8080")?
			.run()
			.await;

		println!("Ok.");

		return Ok(());
	}
}
