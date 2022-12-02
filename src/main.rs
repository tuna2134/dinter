use ring_compat::signature::{
    ed25519::{Signature, VerifyingKey},
    Verifier,
};
use actix_web::{
    web, App, HttpRequest, HttpResponse,
    HttpServer, Responder, middleware::Logger,
};
use env_logger::Env;
use hex_literal::hex;
use const_decoder::Decoder;
use serde_json::json;

struct AppState {
    verifykey: VerifyingKey,
}

async fn base() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn interaction(
    req: HttpRequest, data: web::Data<AppState>, payload: web::Json<serde_json::Value>
) -> impl Responder {
    let sig: &str = req.headers().get("X-Signature-Ed25519").unwrap().to_str().unwrap();
    let sign: [u8; 64] = Decoder::Hex.decode(sig.as_bytes());
    let signature = Signature::from_bytes(&sign).unwrap();
    let timestamp: &str = req.headers().get("X-Signature-Timestamp").unwrap().to_str().unwrap();
    let body = payload.to_string();
    println!("{}", body);
    let result = data.verifykey.verify(
        format!("{}{}", timestamp, body).as_bytes(),
        &signature
    );
    if result.is_ok() {
        println!("ok");
        web::Json(json!({
            "type": 1,
        }))
    } else {
        println!("failed");
        web::Json(json!({
            "error": "failed",
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pk = hex!("07e912b259237e947da9c0e677dd6f8834cd2cf0c4b22cd1ab05fed570473505");
    let verifykey: VerifyingKey = VerifyingKey::new(&pk).unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(base))
            .route("/interaction", web::post().to(interaction))
            .app_data(web::Data::new(AppState {
                verifykey: verifykey.clone(),
            }))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}