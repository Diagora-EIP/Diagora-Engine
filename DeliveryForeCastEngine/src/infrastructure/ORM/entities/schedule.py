# infrastructure/ORM/entities/schedule.py

class Schedule:
    def __init__(self,schedule_id=None, delivery_date=None, itinerary_id=None, actual_delivery_time=None, order_status=None, created_at=None, updated_at=None, order_id=None, user_id=None, estimated_delivery_time=None, forecast=None, client_id=None, schedule_number=None):
        self.schedule_id = schedule_id
        self.delivery_date = delivery_date
        self.itinerary_id = itinerary_id
        self.actual_delivery_time = actual_delivery_time
        self.order_status = order_status
        self.created_at = created_at
        self.updated_at = updated_at
        self.order_id = order_id
        self.user_id = user_id
        self.estimated_delivery_time = estimated_delivery_time
        self.forecast = forecast
        self.client_id = client_id
        self.schedule_number = schedule_number


    @classmethod
    def from_dict(cls, data):
        return cls(
            schedule_id=data.get("schedule_id"),
            delivery_date=data.get("delivery_date"),
            itinerary_id=data.get("itinerary_id"),
            actual_delivery_time=data.get("actual_delivery_time"),
            order_status=data.get("order_status"),
            created_at=data.get("created_at"),
            updated_at=data.get("updated_at"),
            order_id=data.get("order_id"),
            user_id=data.get("user_id"),
            estimated_delivery_time=data.get("estimated_delivery_time"),
            forecast=data.get("forecast"),
            client_id=data.get("client_id"),
            schedule_number=data.get("schedule_number")
        )

    def to_dict(self):
        return {
            "schedule_id": self.schedule_id,
            "delivery_date": self.delivery_date,
            "itinerary_id": self.itinerary_id,
            "actual_delivery_time": self.actual_delivery_time,
            "order_status": self.order_status,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "order_id": self.order_id,
            "user_id": self.user_id,
            "estimated_delivery_time": self.estimated_delivery_time,
            "forecast": self.forecast,
            "client_id": self.client_id,
            "schedule_number": self.schedule_number
        }
