# dependency_injection.py

from infrastructure.services.database_service import Database
from infrastructure.repositories.CompanyRepository import CompanyRepository

class DependencyInjectionContainer:
    def __init__(self):
        self.database_service = Database()
        self.company_repository = CompanyRepository(self.database_service)

    def get_company_repository(self):
        return self.company_repository

    # Add methods to retrieve other repositories as needed

# Create a singleton instance of the container
container = DependencyInjectionContainer()
