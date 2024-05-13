# dependency_injection.py

from infrastructure.services.database_service import Database
from infrastructure.repositories.CompanyRepository import CompanyRepository
from infrastructure.repositories.ScheduleRepository import ScheduleRepository
from infrastructure.repositories.OrderRepository import OrderRepository

class DependencyInjectionContainer:
    def __init__(self):
        self.database_service = Database()
        self.company_repository = CompanyRepository(self.database_service)
        self.schedule_repository = ScheduleRepository(self.database_service)
        self.order_repository = OrderRepository(self.database_service)

    def get_company_repository(self):
        return self.company_repository

    def get_schedule_repository(self):
        return self.schedule_repository

    def get_order_repository(self):
        return self.order_repository

    # Add methods to retrieve other repositories as needed

# Create a singleton instance of the container
container = DependencyInjectionContainer()
