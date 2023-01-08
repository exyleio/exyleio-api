from fastapi import APIRouter

from app.api.v1.endpoints import players, is_redis_online

api_router = APIRouter()

api_router.include_router(is_redis_online.router, tags=["Test"])
api_router.include_router(players.router, prefix="/players", tags=["Players"])
