from typing import Any

from fastapi import APIRouter, HTTPException
from firebase_admin import auth
from firebase_admin._auth_utils import UserNotFoundError
from http import HTTPStatus

from app import models, redis_manager
from app.core import error

router = APIRouter()


def display_name_to_redis(uid: str, username: str):
    """
    Transfer username data from firebase to redis and delete it from firebase
    """

    player_key = f"player:{uid}"

    # create user if it doesn't exist
    if redis_manager.json.get(player_key) is None:
        redis_manager.json.set(player_key, "$", {"uid": uid})

    # set username
    redis_manager.json.set(player_key, "$.username", username)

    # delete from firebase
    auth.update_user(uid, display_name=auth.DELETE_ATTRIBUTE)


@router.get("/{uid}", response_model=models.Player)
def get_player_by_ID(uid: str) -> Any:

    firebase_data: dict = {}

    # fetch user data
    try:
        firebase_data = auth.get_user(uid)._data
    except UserNotFoundError:
        raise HTTPException(
            status_code=HTTPStatus.INTERNAL_SERVER_ERROR,
            detail=error.USER_NOT_FOUND,
        )

    # if displayName is found in user data, transfer it to redis DB
    if "displayName" in firebase_data:
        display_name_to_redis(uid, firebase_data["displayName"])

    redis_data = redis_manager.json.get(f"player:{uid}")

    try:
        return {
            "uid": uid,
            "username": redis_data["username"],
            "email_verified": firebase_data["emailVerified"],
            "last_login": firebase_data["lastLoginAt"],
            "created_at": firebase_data["createdAt"],
            "tags": [],
        }
    except Exception:
        raise HTTPException(
            status_code=HTTPStatus.INTERNAL_SERVER_ERROR,
            detail=error.USERNAME_NOT_FOUND,
        )
