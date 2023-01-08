from fastapi import APIRouter

from app.api.v1.endpoints import players

api_router = APIRouter()

api_router.include_router(players.router, prefix="/players", tags=["Players"])
