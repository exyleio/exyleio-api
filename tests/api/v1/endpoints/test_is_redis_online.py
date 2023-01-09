import requests


def test_is_redis_online():
    response = requests.get("http://localhost/v1/is_redis_online").json()
    assert response == "True"
