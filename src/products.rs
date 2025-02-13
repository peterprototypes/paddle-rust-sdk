use serde::Serialize;

use crate::ids::ProductID;
use crate::PaddleRequest;

/// Returns a paginated list of products. Use the query parameters to page through results.
///
/// By default, Paddle returns products that are active. Use the status query parameter to return products that are archived.
/// Use the include parameter to include related price entities in the response.
#[derive(Serialize, Default)]
pub struct ListProducts {
    pub after: Option<ProductID>,
    #[serde(serialize_with = "crate::comma_separated")]
    pub id: Option<Vec<ProductID>>,
    #[serde(serialize_with = "crate::comma_separated")]
    pub include: Option<Vec<String>>,
}

impl PaddleRequest for ListProducts {
    type Response = Vec<crate::entities::Product>;

    fn path(&self) -> String {
        "/products".to_string()
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
