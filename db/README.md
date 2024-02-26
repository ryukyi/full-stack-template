# DEBUGGING

```bash
docker build -t postgres-debug .
docker run --name meals -e POSTGRES_PASSWORD=mealspassword -d postgres-debug
```