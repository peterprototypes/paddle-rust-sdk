//! Builders for making requests to the Paddle API for previewing prices.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/pricing-preview/overview) documentation for more information.

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::Event;
use crate::ids::PaddleID;
use crate::paginated::Paginated;
use crate::Paddle;

pub trait ReportType: Serialize {
    type FilterName: Serialize + DeserializeOwned;
}

/// Request builder for querying Paddle for events.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct EventsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<PaddleID>,
    order_by: Option<String>,
    per_page: Option<usize>,
}

impl<'a> EventsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            after: None,
            order_by: None,
            per_page: None,
        }
    }

    /// Return entities after the specified Paddle ID when working with paginated endpoints. Used in the `meta.pagination.next` URL in responses for list operations.
    pub fn after(&mut self, report_id: impl Into<PaddleID>) -> &mut Self {
        self.after = Some(report_id.into());
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `id`
    pub fn order_by_asc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[ASC]", field));
        self
    }

    /// Order returned entities by the specified field. Valid fields for ordering: `id`
    pub fn order_by_desc(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(format!("{}[DESC]", field));
        self
    }

    /// Set how many entities are returned per page. Paddle returns the maximum number of results if a number greater than the maximum is requested.
    /// Check `meta.pagination.per_page` in the response to see how many were returned.
    ///
    /// Default: `50`; Maximum: `200`.
    pub fn per_page(&mut self, entities_per_page: usize) -> &mut Self {
        self.per_page = Some(entities_per_page);
        self
    }

    /// Returns a paginator for fetching pages of entities from Paddle
    pub fn send(&self) -> Paginated<'_, Vec<Event>> {
        Paginated::new(self.client, "/events", self)
    }
}
