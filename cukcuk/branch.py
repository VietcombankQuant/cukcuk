from .common import SqlTableBase, SqlTableMixin
from sqlalchemy.orm import Mapped, mapped_column, MappedAsDataclass
from sqlalchemy import String as SqlString, Boolean as SqlBool, Float as SqlFloat


class Branch(SqlTableBase, SqlTableMixin, MappedAsDataclass):
    """
    Read more: https://graphapi.cukcuk.vn/document/api/branchs_setting.html#branch-definition
    """

    __tablename__ = "branches"

    Id:	Mapped[str] = mapped_column(SqlString, primary_key=True)
    Code: Mapped[str] = mapped_column(SqlString)
    Name: Mapped[str] = mapped_column(SqlString)
    IsBaseDepot: Mapped[bool] = mapped_column(SqlBool)
    IsChainBranch: Mapped[bool] = mapped_column(SqlBool)
    HasVATRate: Mapped[bool] = mapped_column(SqlBool)
    VATForDelivery: Mapped[bool] = mapped_column(SqlBool)
    VATForTakeAway: Mapped[bool] = mapped_column(SqlBool)
    VATForShip: Mapped[bool] = mapped_column(SqlBool)
    VATRate: Mapped[float] = mapped_column(SqlFloat)
    HasCalcService: Mapped[bool] = mapped_column(SqlBool)
    CalcServiceDelivery: Mapped[bool] = mapped_column(SqlBool)
    CalcServiceTakeAway: Mapped[bool] = mapped_column(SqlBool)
    IsCalcServiceAmountNotPromotion: Mapped[bool] = mapped_column(SqlBool)
    CalTaxForService: Mapped[bool] = mapped_column(SqlBool)
    HasServiceRate: Mapped[bool] = mapped_column(SqlBool)
    ServiceRate: Mapped[float] = mapped_column(SqlFloat)
    HasAmountService: Mapped[bool] = mapped_column(SqlBool)
    AmountService: Mapped[float] = mapped_column(SqlFloat)
