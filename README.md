# Paddle Rust SDK

[![crates.io](https://img.shields.io/crates/v/paddle-rust-sdk?label=latest)](https://crates.io/crates/paddle-rust-sdk)
[![Docs](https://docs.rs/paddle-rust-sdk/badge.svg)](https://docs.rs/paddle-rust-sdk)
![License](https://img.shields.io/crates/l/paddle-rust-sdk.svg)

Rust SDK for working with the [Paddle](https://www.paddle.com/) API in server-side apps. (Unofficial)

## Paddle API Coverage

The following list outlines the current coverage of the Paddle API in this SDK. Everything in green is usable. Contributions are welcome!

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
- 👷 Pricing preview
- 🚧 Reports
- 🚧 Events
- 🚧 Notifications
- 🚧 Simulations

## Running examples

`<YOUR_API_KEY>` must be generated in the sandbox environment. All examples call the sandbox endpoints.

```bash
PADDLE_API_KEY=<YOUR_API_KEY> cargo run --example products-list
```
