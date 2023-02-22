import os


def spawn():
    os.environ["DATABASE_URL"] = "postgresql://postgres:example@localhost:5432/postgres"
    os.system("cargo run --bin user-service 8076")


if __name__ == "__main__":
    spawn()
