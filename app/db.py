"""
Data models

Database Schema:

| Key                                        | Type       | Field/Member         | Value/Score                                    |
| ------------------------------------------ | ---------- | -------------------- | ---------------------------------------------- |
| rank:<game-mode>                           | Sorted Set | player id            | ranking criteria                               |
| rank:<game-mode>:days-number-one           | Sorted Set | player id            | total number of days player was in the #1 rank |
| rank:<game-mode>:longest-number-one-streak | Sorted Set | player id            | player's longest #1 streak                     |
| rank:xp                                    | Sorted Set | player id            | Player XP                                      |
| player:<player-id>:items                   | Set        | item id              | -                                              |
| player:<id>                                | Hash       | models.player.Player | in-exhaustive player data                      |
"""

import redis
from app.core.config import settings


class RedisManager:
    def __init__(self):
        self.client = redis.from_url(settings.REDIS_ADDRESS)


redis_manager = RedisManager()
