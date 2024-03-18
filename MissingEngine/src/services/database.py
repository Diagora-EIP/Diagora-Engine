class Database:
    def __init__(self):
        import os
        import logging

        from dotenv import load_dotenv
        from supabase import create_client, Client
        load_dotenv()
        self.supabase_url = os.getenv("SUPABASE_URL")
        self.supabase_key = os.getenv("SUPABASE_KEY")
        self.supabase: Client = create_client(self.supabase_url, self.supabase_key)
        logging.getLogger("httpx").setLevel(logging.WARNING)

    def check_db_empty(self, table):
        data = self.supabase.table(table).select('*').execute().data
        if not data:
            return True
        return False


    def insert(self, table, data):
        return self.supabase.table(table).insert(data).execute()

    def find(self, table, typeEq, id):
        return self.supabase.table(table).select('*').eq(typeEq, id).execute()

    def find_if_exist(self, table, typeEq, id):
        data = self.find(table, typeEq, id)
        if data.data:
            return data.data
        return None

    def update(self, table, typeEq, id, data):
        return self.supabase.table(table).update(data).eq(typeEq, id).execute()
    
    def delete(self, table, typeEq, id):
        return self.supabase.table(table).delete().eq(typeEq, id).execute()
    
    