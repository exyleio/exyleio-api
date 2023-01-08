from typing import Any

from fastapi import APIRouter, Depends, HTTPException

from app import models

router = APIRouter()


@router.get("/{player_id}", response_model=models.Player)
def get_player_by_ID(player_id: str) -> Any:
    """
    Get a player by ID.
    """

    return {"id": player_id, "username": "username", "tags": [models.Tag.pomp]}
