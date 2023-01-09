# install firebase CLI if it isn't installed already
if ! command -v firebase &>/dev/null; then
    npm install -g firebase-tools
fi

# logs in to firebase if it hasn't already
firebase login

# start firebase emulator in the background and continue
firebase emulators:start --only auth &

# start docker containers
docker compose up --build --remove-orphans --pull always
