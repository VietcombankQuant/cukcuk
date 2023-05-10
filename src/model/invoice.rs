use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
struct InvoicePagingParam {
    pub page: u32,
    pub limit: u32,

    #[serde(rename = "BranchID")]
    pub branch_id: String,

    pub have_customer: bool,
    pub last_sync_date: String,
}

struct Invoice {
    RefId: String,
    RefType: u32,
    RefNo: String,
    RefDate: String,
    BranchId: String,
    OrderId: String,
    OrderType: u32,
    ShippingDate: String,
    ShippingDueDate: String,
    CustomerId: String,
    CustomerName: String,
    CustomerTel: String,
    MembershipCardId: String,
    EmployeeId: String,
    EmployeeName: String,
    DeliveryEmployeeId: String,
    DeliveryEmployeeName: String,
    WaiterEmployeeId: String,
    WaiterEmployeeName: String,
    ShippingAddress: String,
    PromotionId: String,
    PromotionName: String,
    TableName: String,
    Description: String,
    DepositAmount: f64,
    Amount: f64,
    DeliveryAmount: f64,
    ServiceRate: f64,
    ServiceAmount: f64,
    VATRate: f64,
    VATAmount: f64,
    DiscountAmount: f64,
    PromotionRate: f64,
    PromotionAmount: f64,
    PromotionItemsAmount: f64,
    ReceiveAmount: f64,
    ReturnAmount: f64,
    TotalAmount: f64,
    SaleAmount: f64,
    TotalItemAmount: f64,
    TotalItemAmountAfterTax: f64,
    TipAmount: f64,
    ServiceTaxRate: f64,
    DeliveryTaxRate: f64,
    CancelDate: String,
    CancelBy: String,
    CancelReason: String,
    PaymentStatus: u32,
    AvailablePoint: u32,
    UsedPoint: i32,
    AddPoint: i32,
}
