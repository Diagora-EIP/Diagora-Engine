# infrastructure/ORM/entities/company.py

class Company:
    def __init__(self, company_id=None, name=None, address=None, forecast_threshold=2, created_at=None, updated_at=None):
        self.company_id = company_id
        self.name = name
        self.address = address
        self.created_at = created_at
        self.updated_at = updated_at
        self.forecast_threshold = forecast_threshold

    @classmethod
    def from_dict(cls, data):
        return cls(
            company_id=data.get("company_id"),
            name=data.get("name"),
            address=data.get("address"),
            created_at=data.get("created_at"),
            updated_at=data.get("updated_at"),
            forecast_threshold=data.get("forecast_threshold")
        )

    def to_dict(self):
        return {
            "company_id": self.company_id,
            "name": self.name,
            "address": self.address,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "forecast_threshold": self.forecast_threshold
        }
