import os

def spawn():
  os.environ["GATEWAY_ADDR"] = "localhost:8080"
  os.system("cargo gateway")

if __name__ == "__main__":
  spawn()