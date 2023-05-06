from typing import Any
from .common import SqlTableBase, SqlTableMixin
from sqlalchemy.orm import mapped_column, mapped_collection
from sqlalchemy import String as SqlString, Boolean as SqlBool, Float as SqlFloat, Integer as SqlInt


class VATInfo(SqlTableBase, SqlTableMixin):
    """
    Read more: https://graphapi.cukcuk.vn/document/api/sainvoices.html#savatinfo-definition
    """

    __tablename__ = "vat_info"

    VATID = mapped_column(SqlString, primary_key=True)
    RefID = mapped_column(SqlString)
    ReceiverEIvoiceName = mapped_column(SqlString)
    Tel = mapped_column(SqlString)
    CompanyName = mapped_column(SqlString)
    CompanyAddress = mapped_column(SqlString)
    TaxCode = mapped_column(SqlString)
    Email = mapped_column(SqlString)
    Status = mapped_column(SqlBool)
    StatusReleaseEInvoice = mapped_column(SqlInt)
    EInvoiceNumber = mapped_column(SqlString)
    StatusSendEmail = mapped_column(SqlInt)
    TransactionID = mapped_column(SqlString)
    SellerTaxCode = mapped_column(SqlString)
    TemplateCode = mapped_column(SqlString)
    InvoiceSeries = mapped_column(SqlString)
    RefDateReleaseEInvoice = mapped_column(SqlString)
    StatusSendToTax = mapped_column(SqlInt)
    AccountObjectIdentificationNumber = mapped_column(SqlString)
    IsCalculatingMachinePublishing = mapped_column(SqlBool)
    ErrorNoteEinvoice = mapped_column(SqlString)


class InvoiceDetail(SqlTableBase, SqlTableMixin):

    __tablename__ = "invoice_details"

    RefDetailId = mapped_column(SqlString, primary_key=True)
    RefID = mapped_column(SqlString)
    RefDetailType = mapped_column(SqlInt)
    ItemID = mapped_column(SqlString)
    ItemName = mapped_column(SqlString)
    Quantity = mapped_column(SqlFloat)
    UnitPrice = mapped_column(SqlFloat)
    UnitID = mapped_column(SqlString)
    UnitName = mapped_column(SqlString)
    Amount = mapped_column(SqlFloat)
    DiscountRate = mapped_column(SqlFloat)
    Description = mapped_column(SqlString)
    SortOrder = mapped_column(SqlInt)
    ParentID = mapped_column(SqlString)
    InventoryItemAdditionID = mapped_column(SqlString)
    InventoryItemType = mapped_column(SqlInt)
    IsSeftPrice = mapped_column(SqlBool)
    PromotionRate = mapped_column(SqlFloat)
    PromotionType = mapped_column(SqlInt)
    PromotionName = mapped_column(SqlString)
    OrderDetailID = mapped_column(SqlString)
    SAInvoicePromotionAmount = mapped_column(SqlFloat)
    RefDate = mapped_column(SqlString)
    ItemCode = mapped_column(SqlString)
    PromotionAmount = mapped_column(SqlFloat)
    InventoryItemCategoryID = mapped_column(SqlString)
    AllocationAmount = mapped_column(SqlFloat)
    PreTaxAmount = mapped_column(SqlFloat)
    TaxRate = mapped_column(SqlFloat)
    TaxAmount = mapped_column(SqlFloat)
    AllocationDeliveryPromotionAmount = mapped_column(SqlFloat)


class InvoicePayment(SqlTableBase, SqlTableMixin):
    """
    Read more: https://graphapi.cukcuk.vn/document/api/sainvoices.html#invoicepayment-definition
    """

    __tablename__ = "invoice_payments"

    SAInvoicePaymentID = mapped_column(SqlString, primary_key=True)
    RefID = mapped_column(SqlString)
    RefNo = mapped_column(SqlString)
    PaymentType = mapped_column(SqlInt)
    Amount = mapped_column(SqlFloat)
    CustomerID = mapped_column(SqlString)
    CustomerName = mapped_column(SqlString)
    PaymentName = mapped_column(SqlString)
    VoucherID = mapped_column(SqlString)
    VoucherQuantity = mapped_column(SqlInt)
    VoucherAmount = mapped_column(SqlFloat)
    VoucherCode = mapped_column(SqlString)
    VoucherName = mapped_column(SqlString)
    CardID = mapped_column(SqlString)
    CardName = mapped_column(SqlString)
    ApplyVoucherType = mapped_column(SqlInt)
    VoucherAllAmount = mapped_column(SqlFloat)
    VoucherFoodAmount = mapped_column(SqlFloat)
    VoucherDrinkAmount = mapped_column(SqlFloat)
    CardNo = mapped_column(SqlString)
    ApprovalCode = mapped_column(SqlString)
    CustomerAddress = mapped_column(SqlString)
    BankName = mapped_column(SqlString)
    BankAccountNumber = mapped_column(SqlString)
    CurrencyID = mapped_column(SqlString)
    MainCurrency = mapped_column(SqlString)
    ExchangeRate = mapped_column(SqlFloat)
    ExchangeAmount = mapped_column(SqlFloat)


class InvoiceCoupon(SqlTableBase, SqlTableMixin):
    """
    Read more: https://graphapi.cukcuk.vn/document/api/sainvoices.html#invoicecoupon-definition
    """

    __tablename__ = "invoice_coupons"

    SAInvoiceCouponID = mapped_column(SqlString, primary_key=True)
    RefID = mapped_column(SqlString)
    CouponID = mapped_column(SqlString)
    CouponCode = mapped_column(SqlString)
    DiscountType = mapped_column(SqlInt)
    DiscountPercent = mapped_column(SqlFloat)
    DiscountAmount = mapped_column(SqlFloat)
    ApplyFromDate = mapped_column(SqlString)
    ApplyToDate = mapped_column(SqlString)
    ApplyCondition = mapped_column(SqlString)
    IsUnlimitedApply = mapped_column(SqlBool)
    ApplyFor = mapped_column(SqlString)
    InvoiceDiscountAmount = mapped_column(SqlFloat)


class Invoice(SqlTableBase, SqlTableMixin):
    """
    Read more: https://graphapi.cukcuk.vn/document/api/sainvoices.html#sainvoice-definition
    """

    __tablename__ = "invoices"

    RefId = mapped_column(SqlString, primary_key=True)
    RefType = mapped_column(SqlInt)
    RefNo = mapped_column(SqlString)
    RefDate = mapped_column(SqlFloat)
    BranchId = mapped_column(SqlString)
    OrderId = mapped_column(SqlString)
    OrderType = mapped_column(SqlInt)
    ShippingDate = mapped_column(SqlFloat)
    ShippingDueDate = mapped_column(SqlFloat)
    CustomerId = mapped_column(SqlString)
    CustomerName = mapped_column(SqlString)
    CustomerTel = mapped_column(SqlString)
    MembershipCardId = mapped_column(SqlString)
    EmployeeId = mapped_column(SqlString)
    EmployeeName = mapped_column(SqlString)
    DeliveryEmployeeId = mapped_column(SqlString)
    DeliveryEmployeeName = mapped_column(SqlString)
    WaiterEmployeeId = mapped_column(SqlString)
    WaiterEmployeeName = mapped_column(SqlString)
    ShippingAddress = mapped_column(SqlString)
    PromotionId = mapped_column(SqlString)
    PromotionName = mapped_column(SqlString)
    TableName = mapped_column(SqlString)
    Description = mapped_column(SqlString)
    DepositAmount = mapped_column(SqlFloat)
    Amount = mapped_column(SqlFloat)
    DeliveryAmount = mapped_column(SqlFloat)
    ServiceRate = mapped_column(SqlFloat)
    ServiceAmount = mapped_column(SqlFloat)
    VATRate = mapped_column(SqlFloat)
    VATAmount = mapped_column(SqlFloat)
    DiscountAmount = mapped_column(SqlFloat)
    PromotionRate = mapped_column(SqlFloat)
    PromotionAmount = mapped_column(SqlFloat)
    PromotionItemsAmount = mapped_column(SqlFloat)
    ReceiveAmount = mapped_column(SqlFloat)
    ReturnAmount = mapped_column(SqlFloat)
    TotalAmount = mapped_column(SqlFloat)
    SaleAmount = mapped_column(SqlFloat)
    TotalItemAmount = mapped_column(SqlFloat)
    TotalItemAmountAfterTax = mapped_column(SqlFloat)
    TipAmount = mapped_column(SqlFloat)
    ServiceTaxRate = mapped_column(SqlFloat)
    DeliveryTaxRate = mapped_column(SqlFloat)
    CancelDate = mapped_column(SqlFloat)
    CancelBy = mapped_column(SqlString)
    CancelReason = mapped_column(SqlString)
    PaymentStatus = mapped_column(SqlInt)
    AvailablePoint = mapped_column(SqlInt)
    UsedPoint = mapped_column(SqlInt)
    AddPoint = mapped_column(SqlInt)
    SAInvoiceDetails = mapped_collection(InvoiceDetail)
    SAInvoicePayments = mapped_collection(InvoicePayment)
    SAInvoiceCoupons = mapped_collection(InvoiceCoupon)
    SAVATInfo = mapped_collection(VATInfo)

    def __init__(self):
        super().__init__()
        self.SAInvoiceDetails = []
