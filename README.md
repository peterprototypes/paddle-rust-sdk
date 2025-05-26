# Paddle Rust SDK

[![crates.io](https://img.shields.io/crates/v/paddle-rust-sdk?label=latest)](https://crates.io/crates/paddle-rust-sdk)
[![Docs](https://docs.rs/paddle-rust-sdk/badge.svg)](https://docs.rs/paddle-rust-sdk)
![License](https://img.shields.io/crates/l/paddle-rust-sdk.svg)

Rust SDK for working with the [Paddle](https://www.paddle.com/) API in server-side apps. (Unofficial)

## Paddle API Coverage

The following list outlines the current coverage of the Paddle API in this crate.

- ✅ Products
- ✅ Prices
- ✅ Discounts
- ✅ Customers
- ✅ Addresses
- ✅ Businesses
- ✅ Payment methods
- ✅ Customer portal sessions
- ✅ Transactions
- ✅ Subscriptions
- ✅ Adjustments
- ✅ Pricing preview
- ✅ Reports
- ✅ Events
- 🚧 Notifications
- 🚧 Simulations

## Webhook signature verification

This crate provides a method to verify that received events are genuinely sent from Paddle.

Example verifying the signature of an incoming request (Actix web):

```rust
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use paddle_rust_sdk::{webhooks::MaximumVariance, Paddle};

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let key = "...";

    match Paddle::verify(request_body, key, signature, MaximumVariance::default()) {
        Ok(_) => {
            // Proccess the request asynchronously
            actix_web::rt::spawn(async { dbg!("here") });
        }
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::BadRequest();
        }
    };

    // Respond as soon as possible
    HttpResponse::Ok()
}

```

## Running examples

`<YOUR_API_KEY>` must be generated in the sandbox environment. All examples call the sandbox endpoints.

```bash
PADDLE_API_KEY=<YOUR_API_KEY> cargo run --example products-list
```
