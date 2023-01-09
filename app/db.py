"""# noqa E501
Database Schema:

| Key                                        | Type       | Field/Member         | Value/Score                                    |
| ------------------------------------------ | ---------- | -------------------- | ---------------------------------------------- |
| rank:<game-mode>                           | Sorted Set | player uid           | player RP                                      |
| rank:<game-mode>:days-number-one           | Sorted Set | player uid           | total number of days player was in the #1 rank |
| rank:<game-mode>:longest-number-one-streak | Sorted Set | player uid           | player's longest #1 streak                     |
| rank:xp                                    | Sorted Set | player uid           | Player XP                                      |
| player:<uid>                               | JSON       | models.player.Player | in-exhaustive player data                      |
| usernames                                  | Set        | player username      | -                                              |
"""

import redis
from app.core.config import settings


class RedisManager:
    def __init__(self):
        self.client = redis.from_url(settings.REDIS_ADDRESS)
        self.json = self.client.json()


redis_manager = RedisManager()
