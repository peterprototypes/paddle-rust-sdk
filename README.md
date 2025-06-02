# Paddle Rust SDK

[![crates.io](https://img.shields.io/crates/v/paddle-rust-sdk)](https://crates.io/crates/paddle-rust-sdk)
[![Docs](https://docs.rs/paddle-rust-sdk/badge.svg)](https://docs.rs/paddle-rust-sdk)
![License](https://img.shields.io/crates/l/paddle-rust-sdk.svg)

Rust SDK for working with the [Paddle](https://www.paddle.com/) API in server-side apps. (Unofficial)

## Installation and Usage

To install the Paddle Rust SDK, run the following Cargo command in your project directory:
```sh
cargo add paddle-rust-sdk
```

To authenticate, you'll need an API key. You can create and manage API keys in **Paddle > Developer tools > Authentication**.

Pass your API key while initializing a new Paddle client.

```rust
use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();
}

```

## Fetching Entities

You can list supported entities with the `*-list()` builders on the [Paddle client](https://docs.rs/paddle-rust-sdk/latest/paddle_rust_sdk/struct.Paddle.html). It returns an iterator-like struct to help when working with multiple pages.

Example for customers:

```rust
use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.customers_list();
    let mut paginated = list.per_page(2).send();

    while let Some(page) = paginated.next().await.unwrap() {
        dbg!(page.data);
    }
}
```

Additionally all entities can be fetched via the `.all()` method.

```rust
use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.customers_list();
    let mut paginated = list.per_page(1).send();
    let customers = paginated.all().await.unwrap();

    dbg!(customers);
}

```

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

## License

* Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)