from pydantic import BaseModel


class Player(BaseModel):
    """
    Contains all information about a Player
    """

    # A unique and unchanging identifier of the player
    id: str
    # A unique but variable identifier of the player
    username: str
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
    # tags List[Tag]
    # titles str
    # xp str
    # <game_mode>_stats: str
