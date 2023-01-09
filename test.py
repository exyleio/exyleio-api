#!/usr/bin/env python3

import signal
import pytest
from pytest import ExitCode
import requests
from time import sleep
from subprocess import Popen
from os import killpg, getpgid, setsid


p = Popen(["./run.sh"], shell=True, preexec_fn=setsid)

# wait until firebase emulator is ready
while True:
    try:
        res = requests.get("http://localhost:9099").json()

        if res["authEmulator"]["ready"]:
            break
    except requests.exceptions.ConnectionError:
        sleep(0.5)


# start test without capturing outputs
result = pytest.main(["--capture=no"])

# stop server
killpg(getpgid(p.pid), signal.SIGTERM)

# notify user when it's finished
while True:
    print(
        "All tests passed." if result == ExitCode.OK else "Some tests failed.",
        "Press Ctrl+C to escape",
    )
    sleep(0.8)
