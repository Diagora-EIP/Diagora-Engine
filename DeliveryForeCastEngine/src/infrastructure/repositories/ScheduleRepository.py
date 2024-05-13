from infrastructure.services.database_service import Database
import logging

logging.getLogger("httpx").setLevel(logging.WARNING)

class ScheduleRepository:
    def __init__(self, database: Database):

        # print("selfdb", database_service.testing())
        self.db = database.get_client()

    def find_by_id(self, schedule_id: str):
        try:
            result = self.db.table("schedule").select("*").eq("schedule_id", schedule_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to find schedule by ID: {e}")
    
    def find_all(self):
        try:
            result = self.db.table("schedule").select("*").execute()
            if isinstance(result, dict) and 'error' in result:
                raise Exception(f"Failed to retrieve schedules: {result['error']}")
            schedules = result.data if result else []  # Initialize schedules with an empty list if result is None
            return schedules
        except Exception as e:
            raise Exception(f"Failed to find all schedules: {e}")

    def find_schedule_by_company_id(self, company_id):
        """
        Find schedules by company ID.

        Args:
        - company_id (int): Company ID to search schedules for.

        Returns:
        - list: List of schedules for the given company ID.
        """
        try:
            # Fetch orders based on the company ID
            orders_response = self.db.table("order").select("*").eq("company_id", company_id).execute()

            # if orders_response.status_code != 200:
            #     raise Exception(f"Failed to fetch orders. Status code: {orders_response.status_code}")

            orders = orders_response.data

            # Extract order IDs from the fetched orders
            order_ids = [order['order_id'] for order in orders]

            if not order_ids:
                return []

            # Fetch schedules associated with the extracted order IDs
            schedule_response = self.db.table("schedule").select("*").in_("order_id", order_ids).execute()

            # if schedule_response.status_code != 200:
            #     raise Exception(f"Failed to fetch schedules. Status code: {schedule_response.status_code}")

            schedules = schedule_response.data

            # Assuming order_date is a field in the order data
            for schedule in schedules:
                order_id = schedule.get("order_id")
                # Assuming orders is a list of dictionaries where each dictionary represents an order
                order = next((order for order in orders if order.get("order_id") == order_id), None)
                if order:
                    schedule["order_date"] = order.get("order_date")
                    schedule["description"] = order.get("description")
                    schedule["delivery_address"] = order.get("delivery_address")

            return schedules
        except Exception as e:
            raise Exception(f"Failed to find schedule by company ID: {e}")

    def insert(self, data):
        try:
            result = self.db.table("schedule").insert(data).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to insert schedule: {e}")

    def update(self, schedule_id: str, data):
        try:
            result = self.db.table("schedule").update(data).eq("schedule_id", schedule_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to update schedule: {e}")

    def delete(self, schedule_id: str):
        try:
            result = self.db.table("schedule").delete().eq("schedule_id", schedule_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to delete schedule: {e}")
    
    def get_repetitive_delivery_count(self, schedule_id: int):
        try:
            result = 2
            # if result.error:
            #     raise Exception(result.error)
            return result
        except Exception as e:
            raise Exception(f"Failed to get repetitive delivery count: {e}")
