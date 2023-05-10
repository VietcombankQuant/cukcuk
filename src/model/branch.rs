use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct BranchSummary {
    pub id: String,
    pub code: String,
    pub name: String,
    pub is_base_depot: Option<bool>,
    pub is_chain_branch: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct Branch {
    pub id: String,
    pub code: String,
    pub name: String,
    pub is_base_depot: bool,
    pub is_chain_branch: bool,

    #[serde(rename = "HasVATRate")]
    pub has_vat_rate: bool,

    #[serde(rename = "VATForDelivery")]
    pub vat_for_delivery: bool,

    #[serde(rename = "VATForTakeAway")]
    pub vat_for_takeaway: bool,

    #[serde(rename = "VATForShip")]
    pub vat_for_ship: bool,

    #[serde(rename = "VATRate")]
    pub vat_rate: f32,
    pub has_calc_service: bool,
    pub calc_service_delivery: bool,
    pub calc_service_take_away: bool,
    pub is_calc_service_amount_not_promotion: bool,
    pub cal_tax_for_service: bool,
    pub has_service_rate: bool,
    pub service_rate: f32,
    pub has_amount_service: bool,
    pub amount_service: f32,
}
