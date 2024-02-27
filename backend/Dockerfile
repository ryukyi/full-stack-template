# Build Stage
FROM rust:1.70.0-slim AS builder
WORKDIR /usr/src/app

# Copy the source code and build the application
COPY . .
RUN cargo build --release

# TODO: optimize builder so container is smallest possible - maybe alpine linux?

# Expose the application's port
EXPOSE 8080

# Run the application
CMD ["/usr/src/app/target/release/meal_backend"]