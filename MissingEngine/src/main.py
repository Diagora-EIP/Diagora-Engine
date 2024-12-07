import sys
import dotenv
import requests
import json

import services.file_descriptor as file_descriptor
from services.database import Database
from datetime import datetime

def create_itinary(adress, start_adress, return_to_start):
    payload = {
    "address": adress,
    "startAdress": start_adress,
    "returnToStart": return_to_start
    }
    url = 'http://localhost:9876/launch_itinary/'
    headers = {'Content-Type': 'application/json'}
    
    response = requests.post(url, headers=headers, json=payload)
    data = response.json()
    return data

dotenv.load_dotenv()

if len(sys.argv) < 2:
    print("Usage: python main.py <path to file>")
    sys.exit(1)

filepath = sys.argv[1]
content = file_descriptor.read_file(filepath)
jsonContent = file_descriptor.json_decrypt(content)

missing_user_id = jsonContent['missing_user_id']
day_of_absence_date = datetime.strptime(jsonContent['day_of_absence'], "%Y-%m-%d").date()


db = Database();

db.find('itinerary', 'itinerary_id', 56)

missing_user = db.find('users', 'user_id', missing_user_id).data[0]
user_of_company = db.find('users', 'company_id', missing_user['company_id']).data


# delete missing_user form user_of_company
for user in user_of_company:
    if user['user_id'] == missing_user_id:
        user_of_company.remove(user)

if len(user_of_company) == 0:
    file_descriptor.write_file(filepath + "_result", "No user in the company")
    print(filepath + '_result')
    sys.exit(1)



schedule = db.find("schedule", "user_id", missing_user_id).data
matching_schedule = [obj for obj in schedule if datetime.fromisoformat(obj["delivery_date"]).date() == day_of_absence_date]

schedule_of_users_in_company = []

for user in user_of_company:
    schedules = db.find("schedule", "user_id", user['user_id']).data
    schedule_of_users_in_company.append({
        "user_id": user['user_id'], 
        "schedule": [obj for obj in schedules if datetime.fromisoformat(obj["delivery_date"]).date() == day_of_absence_date]
    })

change_schedule = False

# if one of the user in the company has a emply schedule on the day of absence of the missing user change the user id of the schedule of the missing user to the user id of the user with the empty schedule
for user in schedule_of_users_in_company:
    if len(user['schedule']) == 0:
        for obj in matching_schedule:
            obj['user_id'] = user['user_id']
        change_schedule = True
        break

if change_schedule:
    for obj in matching_schedule:
        obj['proposition'] = True
        db.update("schedule", "schedule_id", obj['schedule_id'], obj)
    file_descriptor.write_file(filepath + "_result", content)
    print(filepath + '_result')
    sys.exit(0)

# Sort users by the number of schedules they have on the day of absence
sorted_users = sorted(schedule_of_users_in_company, key=lambda x: len(x['schedule']))

# Assign schedules one by one from the missing user to users in the company with the least schedules
for obj in matching_schedule:
    for user in sorted_users:
        obj['user_id'] = user['user_id']
        obj['itinerary_id'] = user['schedule'][0]["itinerary_id"]
        user['schedule'].append(obj)
        sorted_users = sorted(sorted_users, key=lambda x: len(x['schedule']))  # Re-sort users after assignment
        break

# update itinary of the users in the company
for user in schedule_of_users_in_company:
    orders = []
    for obj in user['schedule']:
        temp_order = db.find("order", "order_id", obj['order_id']).data[0]
        temp_order['delivery_date'] = obj['delivery_date']
        orders.append(temp_order)
        # sort the orders of the user by the created_at date
    orders = sorted(orders, key=lambda x: x['delivery_date'])
    # get all thes adress in orders and put them in a list of {adress: "adress"}
    start_adress = {"address": orders[0]['delivery_address'], "start_at": datetime.fromisoformat(orders[0]["delivery_date"]).timestamp()}
    orders.pop(0)
    adress = []
    for order in orders:
        adress.append({"address": order['delivery_address']})
    return_to_start = False
    if len(adress) == 0:
        continue
    itinary = create_itinary(adress, start_adress, return_to_start)

        

# update the schedule of the missing user
for obj in matching_schedule:
    obj['proposition'] = True
    db.update("schedule", "schedule_id", obj['schedule_id'], obj)

file_descriptor.write_file(filepath + "_result", "Propositions have been made for the missing user.")
print(filepath + '_result')
