use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

const DEFAULT_PORT: u16 = 12345;
const DEFAULT_HOST: &'static str = "127.0.0.1";

#[derive(Deserialize)]
struct GitHubWebhookPayload {
    zen: String
}

fn handle_event() {
    let _ = run_script::run_script!(
        r#"
            echo "Do something here"
        "#
    ).expect("Failed to handle event!");
}

#[post("/webhook")]
async fn handle_webhook_post(req_body: String) -> impl Responder {
    match serde_json::from_str::<GitHubWebhookPayload>(&req_body) {
        Ok(payload) => println!("zen: {}", payload.zen),
        Err(_) => {}
    }
    handle_event();
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(handle_webhook_post) })
        .bind((DEFAULT_HOST, DEFAULT_PORT))?
        .run()
        .await
}