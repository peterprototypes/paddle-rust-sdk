use crate::{Error, Paddle, SuccessResponse};
use reqwest::{Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::marker::PhantomData;

pub struct Paginated<'a, T> {
    client: &'a Paddle,
    path: String,
    query: Option<Value>,
    _type: PhantomData<T>,
}

impl<'a, T> Paginated<'a, T> {
    pub fn create<Q>(client: &'a Paddle, path: &str, query: &Q) -> Result<Self, Error>
    where
        Q: Serialize,
    {
        let query = serde_json::to_value(&query)?;
        Ok(Self {
            client,
            path: path.to_string(),
            query: Some(query),
            _type: PhantomData,
        })
    }
}

impl<'a, T> Paginated<'a, T>
where
    T: DeserializeOwned,
{
    pub async fn next(&mut self) -> Result<Option<SuccessResponse<T>>, Error> {
        if let Some(query) = self.query.take() {
            let response = self.client.send(query, Method::GET, &self.path).await?;
            if let Some(pagination) = &response.meta.pagination {
                if pagination.has_more {
                    let url = Url::parse(&pagination.next)?;
                    self.path = url.path().to_string();
                    let query = url
                        .query()
                        .map(serde_qs::from_str)
                        .transpose()?
                        .unwrap_or_default();
                    self.query = Some(query);
                }
            }
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}
