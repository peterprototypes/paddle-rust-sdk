use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use paddle_rust_sdk::{webhooks::MaximumVariance, Paddle};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    HttpServer::new(|| App::new().service(paddle_callback))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/// http://127.0.0.1:8080/paddle-callback
#[post("/paddle-callback")]
async fn paddle_callback(request_body: String, req: HttpRequest) -> impl Responder {
    let maybe_signature = req
        .headers()
        .get("paddle-signature")
        .and_then(|h| h.to_str().ok());

    let Some(signature) = maybe_signature else {
        return HttpResponse::BadRequest();
    };

    let key = "pdl_ntfset_01jw5t7njm3zfttyc8svst87rm_8ez0Wfm7VaeV+2IT3MpLGxwiQpDHWbYC";

    match Paddle::unmarshal(request_body, key, signature, MaximumVariance::default()) {
        Ok(event) => {
            // Proccess the request asynchronously
            actix_web::rt::spawn(async { dbg!(event) });
        }
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::BadRequest();
        }
    };

    // Respond as soon as possible
    HttpResponse::Ok()
}
