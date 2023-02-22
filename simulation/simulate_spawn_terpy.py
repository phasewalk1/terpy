import multiprocessing
import time

from spawn_gateway import spawn as spawn_gateway
from spawn_grower_service import spawn as spawn_grower_service
from spawn_user_service import spawn as spawn_user_service

if __name__ == "__main__":
    user_service_proc = multiprocessing.Process(target=spawn_user_service)
    user_service_proc.start()
    grower_service_proc = multiprocessing.Process(target=spawn_grower_service)
    grower_service_proc.start()
    gateway_proc = multiprocessing.Process(target=spawn_gateway)
    gateway_proc.start()
    time.sleep(3)
