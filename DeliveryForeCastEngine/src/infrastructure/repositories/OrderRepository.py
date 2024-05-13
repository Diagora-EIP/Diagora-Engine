from infrastructure.services.database_service import Database

class OrderRepository:
    def __init__(self, database: Database):

        # print("selfdb", database_service.testing())
        self.db = database.get_client()

    def find_by_id(self, order_id: str):
        try:
            result = self.db.table("order").select("*").eq("order_id", order_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to find order by ID: {e}")
    
    def find_all(self):
        try:
            result = self.db.table("order").select("*").execute()
            if isinstance(result, dict) and 'error' in result:
                raise Exception(f"Failed to retrieve orders: {result['error']}")
            orders = result.data if result else []  # Initialize orders with an empty list if result is None
            return orders
        except Exception as e:
            raise Exception(f"Failed to find all orders: {e}")

    def insert(self, data):
        try:
            result = self.db.table("order").insert(data).execute()
            # if result.error:
            #     raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to insert order: {e}")

    def update(self, order_id: str, data):
        try:
            result = self.db.table("order").update(data).eq("order_id", order_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to update order: {e}")

    def delete(self, order_id: str):
        try:
            result = self.db.table("order").delete().eq("order_id", order_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to delete order: {e}")
    
    def get_repetitive_delivery_count(self, order_id: int):
        try:
            result = 3
            # if result.error:
            #     raise Exception(result.error)
            return result
        except Exception as e:
            raise Exception(f"Failed to get repetitive delivery count: {e}")
