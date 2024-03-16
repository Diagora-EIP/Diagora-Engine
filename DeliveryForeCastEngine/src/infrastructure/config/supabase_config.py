import os
from dotenv import load_dotenv
from supabase import create_client, Client

class SupabaseClient:
    def __init__(self):
        load_dotenv()
        self.supabase_url = os.environ.get("SUPABASE_URL")
        self.supabase_api_key = os.environ.get("SUPABASE_API_KEY")

        # Check if environment keys are defined
        if not self.supabase_url or not self.supabase_api_key:
            raise ValueError("SUPABASE_URL and SUPABASE_API_KEY must be set in the environment")

        self.client = create_client(self.supabase_url, self.supabase_api_key)
    
    def get_client(self):
        return self.client
