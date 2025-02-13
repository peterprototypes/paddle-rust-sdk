use paddle_rust_sdk::ids::ProductID;
use paddle_rust_sdk::Paddle;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Paddle::new(
        "b4a594c6a0acdad7b84d817d9c0ef4f5117032dd87e27bb099".to_string(),
        Paddle::SANDBOX,
    )
    .unwrap();

    let req = paddle_rust_sdk::products::ListProducts {
        after: None,
        // id: Some(vec![ProductID("pro_01jk83xwchwd87hnywwqwhe1jt".into())]),
        include: Some(vec!["pricesa".to_string()]),
        ..Default::default()
    };

    let products = client.send(req).await.unwrap();

    dbg!(products);

    Ok(())
}
