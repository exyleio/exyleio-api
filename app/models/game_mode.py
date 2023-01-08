from pydantic import BaseModel
from typing import Literal

from .tier import Tier


GameMode = Literal[
    "battle_royale",
    "capture_points",
    "custom",
    "prop_hunt",
    "solo_death_match",
    "speedrun",
    "team_death_match",
    "tutorial",
    "vehicles",
]


class BaseGameModeStats(BaseModel):
    """
    Common statistics across all game modes
    """

    days_number_one: int
    longest_number_one: int
    percentile: int
    percentile_peak: int
    tier: Tier
    tier_peak: Tier
    rank: int
    rank_peak: int
