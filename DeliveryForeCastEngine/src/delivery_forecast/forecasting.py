import time
from infrastructure.dependency_injection import container
from infrastructure.ORM.entities.company import Company
from infrastructure.ORM.entities.schedule import Schedule
from datetime import datetime, timedelta

def perform_forecast():
    """
    Perform forecasting for deliveries based on repetitive deliveries.
    
    This function retrieves repetitive delivery count from the database for each company,
    identifies repetitive deliveries, generates predictions, and logs the time taken for each company.
    """

    try:
        # Retrieve company repository from dependency injection container
        company_repository = container.get_company_repository()
        schedule_repository = container.get_schedule_repository()
        
        # Retrieve all companies from the database
        companies_req = company_repository.find_all()
        companies = [Company(**company) for company in companies_req]

        # Iterate over each company
        for company in companies:
            start_time = time.time()  # Start time for logging

            # Log company name
            # print(f"Processing deliveries for company: {company.name}")
            # print(company.company_id)
            # Get repetitive delivery count for the current company (assuming it's fetched from the database)
            repetitive_delivery_count = company.forecast_threshold
            print(f"Repetitive delivery count for company {company.name}: {repetitive_delivery_count}")

            # Retrieve deliveries for the current company (assuming it's fetched from the database)
            deliveries = schedule_repository.find_schedule_by_company_id(company.company_id)
            # print(deliveries)

            # Identify repetitive deliveries
            if (deliveries == []):
                print(company)
                print("No deliveries found for company: ", company.name)
                continue

            repetitive_deliveries = identify_repetitive_deliveries(deliveries, repetitive_delivery_count)
            if (len(repetitive_deliveries) != 0):
                print(f"\nRepetitive deliveries found for company: {company.name}")
                print("repetitive deliveries ", repetitive_deliveries)

                # Generate predictions for the identified repetitive deliveries
                most_recent_delivery = find_most_recent_delivery(repetitive_deliveries)
                generate_predictions(most_recent_delivery, company.company_id)

            # Log time taken for processing and prediction generation for the current company
            end_time = time.time()
            print(f"Time taken for {company.name}: {end_time - start_time} seconds")

        return
    except Exception as e:
        print(e)
        return

def extract_hour_minute(order_date):
    """
    Extract hour and minute from a delivery date string.
    
    Args:
    - order_date (str): Delivery date string.
    
    Returns:
    - str: Hour and minute portion of the delivery date.
    """
    return order_date.split('T')[1][:5]

def identify_repetitive_deliveries(deliveries, repetitive_delivery_count):
    """
    Identify repetitive deliveries based on the provided repetitive delivery count.
    
    Args:
    - deliveries (list): List of delivery objects.
    - repetitive_delivery_count (int): Threshold to consider it a repetitive delivery.
    
    Returns:
    - list: List of dictionaries containing delivery information for the repetitive dates found.
    """
    # Dictionary to store counts of day of the week, hours, minutes, and client ID
    match_counts = {}

    # Iterate through deliveries and count occurrences of day of the week, hours, minutes, and client ID
    for delivery in deliveries:
        order_date = datetime.fromisoformat(delivery['order_date'])
        day_of_week = order_date.strftime('%A')  # Extract day of the week
        hour_minute = order_date.strftime('%H:%M')  # Extract hours and minutes
        client_id = delivery['client_id']

        if not client_id:
            continue

        key = (day_of_week, hour_minute, client_id)  # Create a tuple for matching key
        if key in match_counts:
            match_counts[key]['count'] += 1
            match_counts[key]['deliveries'].append(delivery)
        else:
            match_counts[key] = {'count': 1, 'deliveries': [delivery]}

    # Filter deliveries that have redundant day of the week, hours, minutes, and client ID
    repetitive_deliveries = []
    for key, data in match_counts.items():
        if data['count'] >= repetitive_delivery_count:
            print(f"Repetitive delivery dates for client {key[2]} at {key[0]} {key[1]}:")
            repetitive_dates = [delivery for delivery in data['deliveries']]
            print([delivery['order_date'] for delivery in repetitive_dates])
            repetitive_deliveries.extend(repetitive_dates)

    return repetitive_deliveries

def find_most_recent_delivery(repetitive_deliveries):
    # Find the most recent delivery based on delivery date
    most_recent_delivery = max(repetitive_deliveries, key=lambda x: datetime.strptime(x['delivery_date'], '%Y-%m-%dT%H:%M:%S%z'))
    return most_recent_delivery

def generate_predictions(most_recent_delivery, company_id: int):
    # Parse most recent delivery date
    most_recent_delivery_date = datetime.strptime(most_recent_delivery['delivery_date'], '%Y-%m-%dT%H:%M:%S%z')
    schedule_repository = container.get_schedule_repository()
    order_repository = container.get_order_repository()
    
    # Calculate next occurrence of the same day of the week, one week later
    next_week_delivery_date = most_recent_delivery_date + timedelta(weeks=1)
    
    # Create a forecast delivery with similar description and time of delivery
    print("\n Most recent delivery: ", most_recent_delivery)
    #Schedule :
        #delivery_date: schedule.delivery_date,
        # order: { order_id: schedule.order.order_id },
        # user: { user_id: schedule.user.id },
        # itinerary_id: schedule.itinerary_id,
        # estimated_time: schedule.estimated_time,
        # actual_time: schedule.actual_time,
        # status: schedule.status,
    #Order :
        # order_date: order.order_date,
        # company: { company_id: order.company.id },
        # delivery_address: order.delivery_address,
        # description: order.description,
    order = {
        'order_date': next_week_delivery_date.strftime('%Y-%m-%dT%H:%M:%S%z'),
        'company_id': company_id,
        'delivery_address': most_recent_delivery['delivery_address'],
        'description': '[Prediction] ' + most_recent_delivery['description'],
        'client_id': most_recent_delivery['client_id'],
    }
    new_order = order_repository.insert(order)
    schedule = {
        'delivery_date': next_week_delivery_date.strftime('%Y-%m-%dT%H:%M:%S%z'),
        'order_id': new_order[0]['order_id'],
        'itinerary_id': 0,
        'user_id': most_recent_delivery['user_id'],
        'client_id': most_recent_delivery['client_id'],
        'forecast': True,
        'order_status': 0
    }
    schedule_repository.insert(schedule)

    print("\n Generarted forecast delivery: ", schedule)

    return schedule

# Entry point
if __name__ == "__main__":
    perform_forecast()
