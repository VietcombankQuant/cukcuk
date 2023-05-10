use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct InvoicePagingParam {
    #[serde(rename = "Page")]
    pub page: u32,
    #[serde(rename = "Limit")]
    pub limit: u32,
    #[serde(rename = "BranchID")]
    pub branch_id: String,
    #[serde(rename = "HaveCustomer")]
    pub have_customer: bool,
    #[serde(rename = "LastSyncDate")]
    pub last_sync_date: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct InvoiceSummary {
    #[serde(rename = "RefId")]
    pub ref_id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Invoice {
    #[serde(rename = "RefId")]
    pub ref_id: String,
    #[serde(rename = "RefType")]
    pub ref_type: i32,
    #[serde(rename = "RefNo")]
    pub ref_no: String,
    #[serde(rename = "RefDate")]
    pub ref_date: String,
    #[serde(rename = "BranchId")]
    pub branch_id: String,
    #[serde(rename = "OrderId")]
    pub order_id: String,
    #[serde(rename = "OrderType")]
    pub order_type: i32,
    #[serde(rename = "ShippingDate")]
    pub shipping_date: String,
    #[serde(rename = "ShippingDueDate")]
    pub shipping_due_date: String,
    #[serde(rename = "CustomerId")]
    pub customer_id: String,
    #[serde(rename = "CustomerName")]
    pub customer_name: String,
    #[serde(rename = "CustomerTel")]
    pub customer_tel: String,
    #[serde(rename = "MembershipCardId")]
    pub membership_card_id: String,
    #[serde(rename = "EmployeeId")]
    pub employee_id: String,
    #[serde(rename = "EmployeeName")]
    pub employee_name: String,
    #[serde(rename = "DeliveryEmployeeId")]
    pub delivery_employee_id: String,
    #[serde(rename = "DeliveryEmployeeName")]
    pub delivery_employee_name: String,
    #[serde(rename = "WaiterEmployeeId")]
    pub waiter_employee_id: String,
    #[serde(rename = "WaiterEmployeeName")]
    pub waiter_employee_name: String,
    #[serde(rename = "ShippingAddress")]
    pub shipping_address: String,
    #[serde(rename = "PromotionId")]
    pub promotion_id: String,
    #[serde(rename = "PromotionName")]
    pub promotion_name: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DepositAmount")]
    pub deposit_amount: f64,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "DeliveryAmount")]
    pub delivery_amount: f64,
    #[serde(rename = "ServiceRate")]
    pub service_rate: f64,
    #[serde(rename = "ServiceAmount")]
    pub service_amount: f64,
    #[serde(rename = "VATRate")]
    pub vatrate: f64,
    #[serde(rename = "VATAmount")]
    pub vatamount: f64,
    #[serde(rename = "DiscountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "PromotionRate")]
    pub promotion_rate: f64,
    #[serde(rename = "PromotionAmount")]
    pub promotion_amount: f64,
    #[serde(rename = "PromotionItemsAmount")]
    pub promotion_items_amount: f64,
    #[serde(rename = "ReceiveAmount")]
    pub receive_amount: f64,
    #[serde(rename = "ReturnAmount")]
    pub return_amount: f64,
    #[serde(rename = "TotalAmount")]
    pub total_amount: f64,
    #[serde(rename = "SaleAmount")]
    pub sale_amount: f64,
    #[serde(rename = "TotalItemAmount")]
    pub total_item_amount: f64,
    #[serde(rename = "TotalItemAmountAfterTax")]
    pub total_item_amount_after_tax: f64,
    #[serde(rename = "TipAmount")]
    pub tip_amount: f64,
    #[serde(rename = "ServiceTaxRate")]
    pub service_tax_rate: f64,
    #[serde(rename = "DeliveryTaxRate")]
    pub delivery_tax_rate: f64,
    #[serde(rename = "CancelDate")]
    pub cancel_date: String,
    #[serde(rename = "CancelBy")]
    pub cancel_by: String,
    #[serde(rename = "CancelReason")]
    pub cancel_reason: String,
    #[serde(rename = "PaymentStatus")]
    pub payment_status: i32,
    #[serde(rename = "AvailablePoint")]
    pub available_point: i32,
    #[serde(rename = "UsedPoint")]
    pub used_point: i32,
    #[serde(rename = "AddPoint")]
    pub add_point: i32,
    #[serde(rename = "SAInvoiceDetails")]
    pub details: Vec<InvoiceDetail>,
    #[serde(rename = "SAInvoicePayments")]
    pub payments: Vec<InvoicePayment>,
    #[serde(rename = "SAInvoiceCoupons")]
    pub coupons: Vec<InvoiceCoupon>,
    #[serde(rename = "SAVATInfo")]
    pub vat_info: VatInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct InvoiceDetail {
    #[serde(rename = "RefDetailId")]
    pub ref_detail_id: String,
    #[serde(rename = "RefID")]
    pub ref_id: String,
    #[serde(rename = "RefDetailType")]
    pub ref_detail_type: i32,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "ItemName")]
    pub item_name: String,
    #[serde(rename = "Quantity")]
    pub quantity: f64,
    #[serde(rename = "UnitPrice")]
    pub unit_price: f64,
    #[serde(rename = "UnitID")]
    pub unit_id: String,
    #[serde(rename = "UnitName")]
    pub unit_name: String,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "DiscountRate")]
    pub discount_rate: f64,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
    #[serde(rename = "ParentID")]
    pub parent_id: String,
    #[serde(rename = "InventoryItemAdditionID")]
    pub inventory_item_addition_id: String,
    #[serde(rename = "InventoryItemType")]
    pub inventory_item_type: i32,
    #[serde(rename = "IsSeftPrice")]
    pub is_seft_price: bool,
    #[serde(rename = "PromotionRate")]
    pub promotion_rate: f64,
    #[serde(rename = "PromotionType")]
    pub promotion_type: i32,
    #[serde(rename = "PromotionName")]
    pub promotion_name: String,
    #[serde(rename = "OrderDetailID")]
    pub order_detail_id: String,
    #[serde(rename = "SAInvoicePromotionAmount")]
    pub sainvoice_promotion_amount: f64,
    #[serde(rename = "RefDate")]
    pub ref_date: String,
    #[serde(rename = "ItemCode")]
    pub item_code: String,
    #[serde(rename = "PromotionAmount")]
    pub promotion_amount: f64,
    #[serde(rename = "InventoryItemCategoryID")]
    pub inventory_item_category_id: String,
    #[serde(rename = "AllocationAmount")]
    pub allocation_amount: f64,
    #[serde(rename = "PreTaxAmount")]
    pub pre_tax_amount: f64,
    #[serde(rename = "TaxRate")]
    pub tax_rate: f64,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: f64,
    #[serde(rename = "AllocationDeliveryPromotionAmount")]
    pub allocation_delivery_promotion_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct InvoicePayment {
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
pub struct InvoiceCoupon {
    #[serde(rename = "SAInvoiceCouponID")]
    pub sainvoice_coupon_id: String,
    #[serde(rename = "RefID")]
    pub ref_id: String,
    #[serde(rename = "CouponID")]
    pub coupon_id: String,
    #[serde(rename = "CouponCode")]
    pub coupon_code: String,
    #[serde(rename = "DiscountType")]
    pub discount_type: i32,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f64,
    #[serde(rename = "DiscountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "ApplyFromDate")]
    pub apply_from_date: String,
    #[serde(rename = "ApplyToDate")]
    pub apply_to_date: String,
    #[serde(rename = "ApplyCondition")]
    pub apply_condition: String,
    #[serde(rename = "IsUnlimitedApply")]
    pub is_unlimited_apply: bool,
    #[serde(rename = "ApplyFor")]
    pub apply_for: String,
    #[serde(rename = "InvoiceDiscountAmount")]
    pub invoice_discount_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct VatInfo {
    #[serde(rename = "VATID")]
    pub vatid: String,
    #[serde(rename = "RefID")]
    pub ref_id: String,
    #[serde(rename = "ReceiverEIvoiceName")]
    pub receiver_eivoice_name: String,
    #[serde(rename = "Tel")]
    pub tel: String,
    #[serde(rename = "CompanyName")]
    pub company_name: String,
    #[serde(rename = "CompanyAddress")]
    pub company_address: String,
    #[serde(rename = "TaxCode")]
    pub tax_code: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Status")]
    pub status: bool,
    #[serde(rename = "StatusReleaseEInvoice")]
    pub status_release_einvoice: i32,
    #[serde(rename = "EInvoiceNumber")]
    pub einvoice_number: String,
    #[serde(rename = "StatusSendEmail")]
    pub status_send_email: i32,
    #[serde(rename = "TransactionID")]
    pub transaction_id: String,
    #[serde(rename = "SellerTaxCode")]
    pub seller_tax_code: String,
    #[serde(rename = "TemplateCode")]
    pub template_code: String,
    #[serde(rename = "InvoiceSeries")]
    pub invoice_series: String,
    #[serde(rename = "RefDateReleaseEInvoice")]
    pub ref_date_release_einvoice: String,
    #[serde(rename = "StatusSendToTax")]
    pub status_send_to_tax: Option<i32>,
    #[serde(rename = "AccountObjectIdentificationNumber")]
    pub account_object_identification_number: String,
    #[serde(rename = "IsCalculatingMachinePublishing")]
    pub is_calculating_machine_publishing: Option<bool>,
    #[serde(rename = "ErrorNoteEinvoice")]
    pub error_note_einvoice: String,
}
