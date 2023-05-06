from dataclasses import dataclass


@dataclass
class Branch:
    Id:	str = None
    Code: str = None
    Name: str = None
    IsBaseDepot: bool = None
    IsChainBranch: bool = None
