from infrastructure.dependency_injection import container
from infrastructure.ORM.entities.company import Company

def perform_forecast():

    try:
        company_repository = container.get_company_repository()
        companies_req = company_repository.find_all()
        companies = [Company(**company) for company in companies_req]
        for company in companies:
            print(company.name)


        #Create new company using Company() and insert it in the db         self.name = name
        # self.address = address
        new_company = Company(name="New Company", address="New Address")
        company_data = {"name": new_company.name, "address": new_company.address}

        # Insert the company data into the database
        # company_repository.insert(company_data)

        company_repository.delete(52)



        return
    except Exception as e:
        print(e)
        return

    return
