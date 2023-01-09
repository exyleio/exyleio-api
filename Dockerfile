FROM python:3.11-slim
VOLUME /app
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
# The Firebase Admin SDK automatically connect to the Authentication emulator
# when the FIREBASE_AUTH_EMULATOR_HOST environment variable is set.
# https://firebase.google.com/docs/emulator-suite/connect_auth#admin_sdks
ENV FIREBASE_AUTH_EMULATOR_HOST="host.docker.internal:9099"
CMD ["uvicorn", "app.main:app", "--reload","--host", "0.0.0.0", "--port", "8000"] 
