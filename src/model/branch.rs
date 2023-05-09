use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct Branch {
    id: String,
    code: String,
    name: String,
    is_base_depot: Option<bool>,
    is_chain_branch: Option<bool>,
}
