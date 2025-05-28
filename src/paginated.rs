use crate::{Error, Paddle, SuccessResponse};
use serde::Serialize;
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

    pub fn next(&mut self) -> Result<SuccessResponse<T>, Error> {
        todo!()
    }
}
