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

Use the `Paddle::unmarshal` method to verify that received events are genuinely sent from Paddle. Additionally, this method returns the deserialized event struct.

Example handling webhook delivery with Actix:

```rust
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
```

This lib also provides the list live and sandbox IPs that webhook requests originate from. 

Use the `Paddle::ALLOWED_WEBHOOK_IPS_PRODUCTION` and `Paddle::ALLOWED_WEBHOOK_IPS_SANDBOX` constants to check that a requests is made from a Paddle server. Actix example:

```rust
#[post("/webhook")]
async fn webhook(req: HttpRequest, post: String) -> Result<impl Responder> {
    // SECURITY: Do not use realip_remote_addr unless you can be sure that the Forwarded and X-Forwarded-For headers cannot be spoofed by the client. If you are running without a proxy then obtaining the peer address [ConnectionInfo::peer_addr] would be more appropriate.
    let maybe_remote_addr = req.connection_info().realip_remote_addr().map(|s| s.to_string());

    let Some(remote_addr) = maybe_remote_addr else {
        return Ok("");
    };

    if !Paddle::ALLOWED_WEBHOOK_IPS_PRODUCTION.contains(&remote_addr.as_str()) {
        return Ok("");
    }

    //...snip
}
```

## Running examples

`<YOUR_API_KEY>` must be generated in the sandbox environment. All examples call the sandbox endpoints.

```bash
PADDLE_API_KEY=<YOUR_API_KEY> cargo run --example products-list
```
