from pydantic import BaseSettings


class Settings(BaseSettings):
    REDIS_ADDRESS: str = "redis://redis:6379"
    API_V1_STR: str = "/v1"


settings = Settings()
