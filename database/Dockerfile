# Use the official PostgreSQL image from the Docker Hub
FROM postgres:latest

# Initialization scripts executed on db start
COPY schema.sql /docker-entrypoint-initdb.d

# Expose the PostgreSQL port
EXPOSE 5432
