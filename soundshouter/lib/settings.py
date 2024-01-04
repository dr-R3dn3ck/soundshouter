from functools import lru_cache

from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    database_path: str = "."
    rabbitmq_url: str = "amqp://localhost:5672"
    rabbitmq_username: str = "guest"
    rabbitmq_password: str = "guest"
    max_consumers: int = 100

    def db_url(self, _async=True) -> str:
        return f"sqlite{'+aiosqlite' if _async else ''}:///{self.database_path}/soundshouter.db"

@lru_cache
def get_settings() -> Settings:
    return Settings()