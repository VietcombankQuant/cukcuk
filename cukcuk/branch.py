from dataclasses import dataclass


@dataclass
class Branch:
    Id:	str = None
    Code: str = None
    Name: str = None
    IsBaseDepot: bool = None
    IsChainBranch: bool = None
    HasVATRate:	bool = None
    VATForDelivery:	bool = None
    VATForTakeAway:	bool = None
    VATForShip:	bool = None
    VATRate: float = None
    HasCalcService:	bool = None
    CalcServiceDelivery: bool = None
    CalcServiceTakeAway: bool = None
    IsCalcServiceAmountNotPromotion: bool = None
    CalTaxForService: bool = None
    HasServiceRate: bool = None
    ServiceRate: float = None
    HasAmountService: bool = None
    AmountService: float = None
