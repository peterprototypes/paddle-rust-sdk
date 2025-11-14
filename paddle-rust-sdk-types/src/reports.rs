use serde::{Serialize, de::DeserializeOwned};

pub trait ReportType: Serialize {
    type FilterName: Serialize + DeserializeOwned;
}
