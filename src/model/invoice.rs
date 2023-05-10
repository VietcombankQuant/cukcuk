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
struct InvoicePayment {
    #[serde(rename = "SAInvoicePaymentID")]
    pub sainvoice_payment_id: String,
    #[serde(rename = "RefID")]
    pub ref_id: String,
    #[serde(rename = "RefNo")]
    pub ref_no: String,
    #[serde(rename = "PaymentType")]
    pub payment_type: i32,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "CustomerID")]
    pub customer_id: String,
    #[serde(rename = "CustomerName")]
    pub customer_name: String,
    #[serde(rename = "PaymentName")]
    pub payment_name: String,
    #[serde(rename = "VoucherID")]
    pub voucher_id: String,
    #[serde(rename = "VoucherQuantity")]
    pub voucher_quantity: i32,
    #[serde(rename = "VoucherAmount")]
    pub voucher_amount: f64,
    #[serde(rename = "VoucherCode")]
    pub voucher_code: String,
    #[serde(rename = "VoucherName")]
    pub voucher_name: String,
    #[serde(rename = "CardID")]
    pub card_id: String,
    #[serde(rename = "CardName")]
    pub card_name: String,
    #[serde(rename = "ApplyVoucherType")]
    pub apply_voucher_type: i32,
    #[serde(rename = "VoucherAllAmount")]
    pub voucher_all_amount: f64,
    #[serde(rename = "VoucherFoodAmount")]
    pub voucher_food_amount: f64,
    #[serde(rename = "VoucherDrinkAmount")]
    pub voucher_drink_amount: f64,
    #[serde(rename = "CardNo")]
    pub card_no: String,
    #[serde(rename = "ApprovalCode")]
    pub approval_code: String,
    #[serde(rename = "CustomerAddress")]
    pub customer_address: String,
    #[serde(rename = "BankName")]
    pub bank_name: String,
    #[serde(rename = "BankAccountNumber")]
    pub bank_account_number: String,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "MainCurrency")]
    pub main_currency: String,
    #[serde(rename = "ExchangeRate")]
    pub exchange_rate: f64,
    #[serde(rename = "ExchangeAmount")]
    pub exchange_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct InvoiceCoupon {
    #[serde(rename = "SAInvoiceCouponID")]
    sainvoice_coupon_id: String,
    #[serde(rename = "RefID")]
    ref_id: String,
    #[serde(rename = "CouponID")]
    coupon_id: String,
    #[serde(rename = "CouponCode")]
    coupon_code: String,
    #[serde(rename = "DiscountType")]
    discount_type: i32,
    #[serde(rename = "DiscountPercent")]
    discount_percent: f64,
    #[serde(rename = "DiscountAmount")]
    discount_amount: f64,
    #[serde(rename = "ApplyFromDate")]
    apply_from_date: String,
    #[serde(rename = "ApplyToDate")]
    apply_to_date: String,
    #[serde(rename = "ApplyCondition")]
    apply_condition: String,
    #[serde(rename = "IsUnlimitedApply")]
    is_unlimited_apply: bool,
    #[serde(rename = "ApplyFor")]
    apply_for: String,
    #[serde(rename = "InvoiceDiscountAmount")]
    invoice_discount_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct VatInfo {
    #[serde(rename = "VATID")]
    vatid: String,
    #[serde(rename = "RefID")]
    ref_id: String,
    #[serde(rename = "ReceiverEIvoiceName")]
    receiver_eivoice_name: String,
    #[serde(rename = "Tel")]
    tel: String,
    #[serde(rename = "CompanyName")]
    company_name: String,
    #[serde(rename = "CompanyAddress")]
    company_address: String,
    #[serde(rename = "TaxCode")]
    tax_code: String,
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Status")]
    status: bool,
    #[serde(rename = "StatusReleaseEInvoice")]
    status_release_einvoice: i32,
    #[serde(rename = "EInvoiceNumber")]
    einvoice_number: String,
    #[serde(rename = "StatusSendEmail")]
    status_send_email: i32,
    #[serde(rename = "TransactionID")]
    transaction_id: String,
    #[serde(rename = "SellerTaxCode")]
    seller_tax_code: String,
    #[serde(rename = "TemplateCode")]
    template_code: String,
    #[serde(rename = "InvoiceSeries")]
    invoice_series: String,
    #[serde(rename = "RefDateReleaseEInvoice")]
    ref_date_release_einvoice: String,
    #[serde(rename = "StatusSendToTax")]
    status_send_to_tax: Option<i32>,
    #[serde(rename = "AccountObjectIdentificationNumber")]
    account_object_identification_number: String,
    #[serde(rename = "IsCalculatingMachinePublishing")]
    is_calculating_machine_publishing: Option<bool>,
    #[serde(rename = "ErrorNoteEinvoice")]
    error_note_einvoice: String,
}
