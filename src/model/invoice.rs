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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceDetail {
    ref_detail_id: String,
    #[serde(rename = "RefID")]
    ref_id: String,
    ref_detail_type: i32,
    #[serde(rename = "ItemID")]
    item_id: String,
    item_name: String,
    quantity: f64,
    unit_price: f64,
    #[serde(rename = "UnitID")]
    unit_id: String,
    unit_name: String,
    amount: f64,
    discount_rate: f64,
    description: String,
    sort_order: i32,
    #[serde(rename = "ParentID")]
    parent_id: String,
    #[serde(rename = "InventoryItemAdditionID")]
    inventory_item_addition_id: String,
    inventory_item_type: i32,
    is_seft_price: bool,
    promotion_rate: f64,
    promotion_type: i32,
    promotion_name: String,
    #[serde(rename = "OrderDetailID")]
    order_detail_id: String,
    #[serde(rename = "SAInvoicePromotionAmount")]
    sa_invoice_promotion_amount: f64,
    ref_date: String,
    item_code: String,
    promotion_amount: f64,
    #[serde(rename = "InventoryItemCategoryID")]
    inventory_item_category_id: String,
    allocation_amount: f64,
    pre_tax_amount: f64,
    tax_rate: f64,
    tax_amount: f64,
    allocation_delivery_promotion_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
struct InvoicePayment {
    #[serde(rename = "SAInvoicePaymentID")]
    pub SAInvoicePaymentID: String,
    #[serde(rename = "RefID")]
    pub RefID: String,
    #[serde(rename = "RefNo")]
    pub RefNo: String,
    #[serde(rename = "PaymentType")]
    pub PaymentType: i32,
    #[serde(rename = "Amount")]
    pub Amount: f64,
    #[serde(rename = "CustomerID")]
    pub CustomerID: String,
    #[serde(rename = "CustomerName")]
    pub CustomerName: String,
    #[serde(rename = "PaymentName")]
    pub PaymentName: String,
    #[serde(rename = "VoucherID")]
    pub VoucherID: String,
    #[serde(rename = "VoucherQuantity")]
    pub VoucherQuantity: i32,
    #[serde(rename = "VoucherAmount")]
    pub VoucherAmount: f64,
    #[serde(rename = "VoucherCode")]
    pub VoucherCode: String,
    #[serde(rename = "VoucherName")]
    pub VoucherName: String,
    #[serde(rename = "CardID")]
    pub CardID: String,
    #[serde(rename = "CardName")]
    pub CardName: String,
    #[serde(rename = "ApplyVoucherType")]
    pub ApplyVoucherType: i32,
    #[serde(rename = "VoucherAllAmount")]
    pub VoucherAllAmount: f64,
    #[serde(rename = "VoucherFoodAmount")]
    pub VoucherFoodAmount: f64,
    #[serde(rename = "VoucherDrinkAmount")]
    pub VoucherDrinkAmount: f64,
    #[serde(rename = "CardNo")]
    pub CardNo: String,
    #[serde(rename = "ApprovalCode")]
    pub ApprovalCode: String,
    #[serde(rename = "CustomerAddress")]
    pub CustomerAddress: String,
    #[serde(rename = "BankName")]
    pub BankName: String,
    #[serde(rename = "BankAccountNumber")]
    pub BankAccountNumber: String,
    #[serde(rename = "CurrencyID")]
    pub CurrencyID: String,
    #[serde(rename = "MainCurrency")]
    pub MainCurrency: String,
    #[serde(rename = "ExchangeRate")]
    pub ExchangeRate: f64,
    #[serde(rename = "ExchangeAmount")]
    pub ExchangeAmount: f64,
}
