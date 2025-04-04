# Paddle Rust SDK
Rust SDK for working with the Paddle API in server-side apps. (Unofficial)

## Paddle API Coverage

The following list outlines the current coverage of the Paddle API in this SDK. Everything in green is usable. Contributions are welcome!

- ✅ Products
- ✅ Prices
- ✅ Discounts
- ✅ Customers
- ✅ Addresses
- 🚧 Businesses
- 🚧 Payment methods
- 🚧 Customer portal sessions
- 🚧 Transactions
- 🚧 Subscriptions
- 🚧 Adjustments
- 🚧 Pricing preview
- 🚧 Reports
- 🚧 Event types
- 🚧 Events
- 🚧 Notification settings
- 🚧 Notifications
- 🚧 Notification logs
- 🚧 Simulations

## Running examples

`<YOUR_API_KEY>` must be generated in the sandbox environment. All examples call the sandbox endpoints.

```bash
PADDLE_API_KEY=<YOUR_API_KEY> cargo run --example products-list
```