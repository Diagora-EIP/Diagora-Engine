from infrastructure.services.database_service import Database

class CompanyRepository:
    def __init__(self, database: Database):

        # print("selfdb", database_service.testing())
        self.db = database.get_client()

    def find_by_id(self, company_id: str):
        try:
            result = self.db.table("company").select("*").eq("company_id", company_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to find company by ID: {e}")
    
    def find_all(self):
        try:
            result = self.db.table("company").select("*").execute()
            if isinstance(result, dict) and 'error' in result:
                raise Exception(f"Failed to retrieve companies: {result['error']}")
            companies = result.data if result else []  # Initialize companies with an empty list if result is None
            return companies
        except Exception as e:
            raise Exception(f"Failed to find all companies: {e}")

    def insert(self, data):
        try:
            result = self.db.table("company").insert(data).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to insert company: {e}")

    def update(self, company_id: str, data):
        try:
            result = self.db.table("company").update(data).eq("company_id", company_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to update company: {e}")

    def delete(self, company_id: str):
        try:
            result = self.db.table("company").delete().eq("company_id", company_id).execute()
            if result.error:
                raise Exception(result.error)
            return result.data
        except Exception as e:
            raise Exception(f"Failed to delete company: {e}")
    
    def get_repetitive_delivery_count(self, company_id: int):
        try:
            result = 3
            # if result.error:
            #     raise Exception(result.error)
            return result
        except Exception as e:
            raise Exception(f"Failed to get repetitive delivery count: {e}")
