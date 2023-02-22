import requests
import json
import multiprocessing
import time

from spawn_gateway import spawn as spawn_gateway
from spawn_grower_service import spawn as spawn_grower_service

GATEWAY_ADDR = "localhost:8000"

if __name__ == "__main__":
    gateway_process = multiprocessing.Process(target=spawn_gateway)
    gateway_process.start()
    grower_service_process = multiprocessing.Process(target=spawn_grower_service)
    grower_service_process.start()

    time.sleep(3)

    url = f"http://{GATEWAY_ADDR}/grower"

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
