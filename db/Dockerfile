# Use the official PostgreSQL image as the base
FROM postgres:latest

# Set environment variables if you need to customize the default user, database, and password
ENV POSTGRES_USER=mealsuser
ENV POSTGRES_PASSWORD=mealspassword
ENV POSTGRES_DB=mealsdb

# Copy your schema.sql file into the container's entrypoint directory for initialization scripts
COPY schema.sql /docker-entrypoint-initdb.d/