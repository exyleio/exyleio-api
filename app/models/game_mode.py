from pydantic import BaseModel
from enum import Enum

from .tier import Tier


class GameMode(Enum):
    battle_royale = 1
    capture_points = 2
    custom = 3
    prop_hunt = 4
    solo_death_match = 5
    speedrun = 6
    team_death_match = 7
    tutorial = 8
    vehicles = 9


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
