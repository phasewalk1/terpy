import os
import requests
import json

service_addr = os.environ.get("GATEWAY_ADDR")

if service_addr is None:
    raise ValueError("GATEWAY_ADDR environment variable is not set")


url = f"http://{service_addr}/grower"

headers = {"Content-Type": "application/json"}

data = {
    "grower_id": "example_grower_id",
    "batch_id": "example_batch_id",
    "cbc": 1.0,
    "cbd": 2.0,
    "cbda": 3.0,
    "cbdv": 4.0,
    "cbg": 5.0,
    "cbga": 6.0,
    "cbn": 7.0,
    "d9thc": 8.0,
    "d8thc": 9.0,
    "thcv": 10.0,
    "thca": 11.0,
}

response = requests.post(url, headers=headers, data=json.dumps(data))

assert response.status_code == 200
print("POST request successful")
print(response.json())
