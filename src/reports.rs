//! Request builders for creating and listing reports.
//!
//! See the [Paddle API](https://developer.paddle.com/api-reference/reports/overview) documentation for more information.

use paddle_rust_sdk_types::reports::ReportType;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::entities::{ReportBase, ReportFilter, ReportFilterValue};
use crate::enums::{FilterOperator, ReportStatus};
use crate::ids::PaddleID;
use crate::paginated::Paginated;
use crate::{Paddle, Result};

/// Request builder for querying Paddle for reports.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ReportsList<'a> {
    #[serde(skip)]
    client: &'a Paddle,
    after: Option<PaddleID>,
    order_by: Option<String>,
    per_page: Option<usize>,
    #[serde(serialize_with = "crate::comma_separated_enum")]
    status: Option<Vec<ReportStatus>>,
}

impl<'a> ReportsList<'a> {
    pub fn new(client: &'a Paddle) -> Self {
        Self {
            client,
            after: None,
            order_by: None,
            per_page: None,
            status: None,
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

    /// Return entities that match the specified status.
    pub fn status(&mut self, statuses: impl IntoIterator<Item = ReportStatus>) -> &mut Self {
        self.status = Some(statuses.into_iter().collect());
        self
    }

    /// Returns a paginator for fetching pages of entities from Paddle
    pub fn send(&self) -> Paginated<'_, Vec<ReportBase>> {
        Paginated::new(self.client, "/reports", self)
    }
}

/// Request builder for creating reports in Paddle.
#[skip_serializing_none]
#[derive(Serialize)]
pub struct ReportCreate<'a, T: ReportType> {
    #[serde(skip)]
    client: &'a Paddle,
    r#type: T,
    filters: Vec<ReportFilter<T::FilterName>>,
}

impl<'a, T: ReportType + DeserializeOwned> ReportCreate<'a, T> {
    pub fn new(client: &'a Paddle, r#type: T) -> Self {
        Self {
            client,
            r#type,
            filters: Vec::new(),
        }
    }

    /// Add filter criteria for this report. If omitted, reports are filtered to include data updated in the last 30 days. This means `updated_at` is greater than or equal to (`gte`) the date 30 days ago from the time the report was generated.
    pub fn append_filter(
        &mut self,
        name: T::FilterName,
        operator: Option<FilterOperator>,
        value: impl Into<ReportFilterValue>,
    ) -> &mut Self {
        self.filters.push(ReportFilter {
            name,
            operator,
            value: value.into(),
        });

        self
    }

    /// Clear all report filters
    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    /// Set all filter criteria for this report. This overrides any previously set filters.
    pub fn set_filters(
        &mut self,
        filters: impl IntoIterator<Item = (T::FilterName, Option<FilterOperator>, ReportFilterValue)>,
    ) -> &mut Self {
        self.filters = filters
            .into_iter()
            .map(|(name, operator, value)| ReportFilter {
                name,
                operator,
                value,
            })
            .collect();
        self
    }

    /// Send the request to Paddle and return the response.
    pub async fn send(&self) -> Result<ReportBase> {
        self.client.send(self, Method::POST, "/reports").await
    }
}
