use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct InvoicePagingParam {
    pub page: u32,
    pub limit: u32,

    #[serde(rename = "BranchID")]
    pub branch_id: String,

    pub have_customer: bool,
    pub last_sync_date: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
    ref_id: String,
    ref_type: i32,
    ref_no: String,
    ref_date: String,
    branch_id: String,
    order_id: String,
    order_type: i32,
    shipping_date: String,
    shipping_due_date: String,
    customer_id: String,
    customer_name: String,
    customer_tel: String,
    membership_card_id: String,
    employee_id: String,
    employee_name: String,
    delivery_employee_id: String,
    delivery_employee_name: String,
    waiter_employee_id: String,
    waiter_employee_name: String,
    shipping_address: String,
    promotion_id: String,
    promotion_name: String,
    table_name: String,
    description: String,
    deposit_amount: f64,
    amount: f64,
    delivery_amount: f64,
    service_rate: f64,
    service_amount: f64,
    #[serde(rename = "VATRate")]
    vat_rate: f64,
    #[serde(rename = "VATAmount")]
    vat_amount: f64,
    discount_amount: f64,
    promotion_rate: f64,
    promotion_amount: f64,
    promotion_items_amount: f64,
    receive_amount: f64,
    return_amount: f64,
    total_amount: f64,
    sale_amount: f64,
    total_item_amount: f64,
    total_item_amount_after_tax: f64,
    tip_amount: f64,
    service_tax_rate: f64,
    delivery_tax_rate: f64,
    cancel_date: String,
    cancel_by: String,
    cancel_reason: String,
    payment_status: i32,
    available_point: i32,
    used_point: i32,
    add_point: i32,
}
