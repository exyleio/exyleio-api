FROM python:3.11-slim
VOLUME /app
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
CMD ["uvicorn", "app.main:app", "--reload","--host", "0.0.0.0", "--port", "8000"] 
