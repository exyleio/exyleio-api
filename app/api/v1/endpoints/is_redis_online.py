from typing import Any

from fastapi import APIRouter

from app.db.redis import redis_manager


router = APIRouter()


@router.get("/is_redis_online", response_model=str)
async def is_redis_online() -> Any:
    return redis_manager.client.ping()
