from pydantic import BaseModel
from .tag import Tag


class Player(BaseModel):
    """
    Contains all information about a Player
    """

    # A unique and unchanging identifier of the player
    uid: str

    # A unique but variable identifier of the player
    username: str

    email_verified: bool

    # UNIX time (millisecond)
    created_at: int

    # UNIX time (millisecond)
    last_login: int

    # badges: str
    # clan: str
    # coins: str
    # discord_id: str
    # friends_count: str
    # game_mode: str
    # items: str
    # join_date: str
    # last_played: str
    # lv: str
    # pfp_url: str
    # play_time_total: str
    tags: list[Tag]
    # titles str
    # xp str
    # <game_mode>_stats: str
