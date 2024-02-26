# Use the official PostgreSQL image from the Docker Hub
FROM postgres:latest

# Set environment variables
ENV POSTGRES_USER=mealsuser
ENV POSTGRES_PASSWORD=mealspassword
ENV POSTGRES_DB=mealsdb

# Expose the PostgreSQL port
EXPOSE 5432

# Optional: Copy initialization scripts if you have any
COPY schema.sql /docker-entrypoint-initdb.d
