import os
from infrastructure.config.supabase_config import SupabaseClient

class Database:
    """
    Class to interact with Supabase database.

    Attributes:
        supabase (Client): Supabase client object for database operations.
    """

    def __init__(self):
        """
        Initializes the Supabase client.
        """
        self.supabase = SupabaseClient().get_client()
    
    def get_client(self):
        """
        Returns the Supabase client object.

        Returns:
            Client: Supabase client object.
        """
        return self.supabase
        

    def check_db_empty(self, table):
        """
        Checks if a given table in the database is empty.

        Args:
            table (str): Name of the table to check.

        Returns:
            bool: True if the table is empty, False otherwise.
        """
        try:
            # Retrieve data from the table
            data = self.supabase.table(table).select('*').execute().data
            # Check if data is empty
            if not data:
                return True
            return False
        except Exception as e:
            raise Exception(f"Failed to check if table '{table}' is empty.") from e

    def insert(self, table, data):
        """
        Inserts data into a specified table in the database.

        Args:
            table (str): Name of the table to insert data into.
            data (dict): Data to be inserted into the table.

        Returns:
            dict: Response from the database operation.
        """
        try:
            return self.supabase.table(table).insert(data).execute()
        except Exception as e:
            raise Exception(f"Failed to insert data into table '{table}'.") from e

    def find(self, table, typeEq, id):
        """
        Finds data in a specified table based on the provided criteria.

        Args:
            table (str): Name of the table to search.
            typeEq (str): Column name to match against.
            id (str): Value to match against.

        Returns:
            dict: Response from the database operation.
        """
        try:
            return self.supabase.table(table).select('*').eq(typeEq, id).execute()
        except Exception as e:
            raise Exception(f"Failed to find data in table '{table}'.") from e

    def find_if_exist(self, table, typeEq, id):
        """
        Checks if data exists in a specified table based on the provided criteria.

        Args:
            table (str): Name of the table to search.
            typeEq (str): Column name to match against.
            id (str): Value to match against.

        Returns:
            dict: Data from the database if found, otherwise None.
        """
        try:
            data = self.find(table, typeEq, id)
            if data.data:
                return data.data
            return None
        except Exception as e:
            raise Exception(f"Failed to check if data exists in table '{table}'.") from e

    def update(self, table, typeEq, id, data):
        """
        Updates data in a specified table based on the provided criteria.

        Args:
            table (str): Name of the table to update.
            typeEq (str): Column name to match against.
            id (str): Value to match against.
            data (dict): Data to be updated.

        Returns:
            dict: Response from the database operation.
        """
        try:
            return self.supabase.table(table).update(data).eq(typeEq, id).execute()
        except Exception as e:
            raise Exception(f"Failed to update data in table '{table}'.") from e
    
    def delete(self, table, typeEq, id):
        """
        Deletes data from a specified table based on the provided criteria.

        Args:
            table (str): Name of the table to delete data from.
            typeEq (str): Column name to match against.
            id (str): Value to match against.

        Returns:
            dict: Response from the database operation.
        """
        try:
            return self.supabase.table(table).delete().eq(typeEq, id).execute()
        except Exception as e:
            raise Exception(f"Failed to delete data from table '{table}'.") from e