from pydantic import BaseSettings


class Settings(BaseSettings):
    # both the API server and the redis server should be running inside docker
    # for this to work
    REDIS_ADDRESS: str = "redis://redis:6379"

    API_V1_STR: str = "/v1"


settings = Settings()
