use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct ServiceResult<T>
where
    T: Debug + Default,
{
    pub code: u32,
    pub error_type: Option<u32>,
    pub error_message: Option<String>,
    pub success: bool,
    pub environment: Option<String>,
    pub data: Option<T>,
    pub total: Option<u32>,
}
