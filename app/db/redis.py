import redis
from app.core.config import settings


class RedisManager:
    def __init__(self):
        self.client = redis.from_url(settings.REDIS_ADDRESS)


redis_manager = RedisManager()
