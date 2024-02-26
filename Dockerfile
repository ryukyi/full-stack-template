# Use the official Rust image as a parent image
FROM rust:1.70.0-slim as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.lock and Cargo.toml files to the container
COPY . .

# Install the application's dependencies and build the application in one step
RUN cargo build --release --manifest-path=backend/Cargo.toml

# Use Alpine for the runtime stage
FROM alpine:3.16.0 AS runtime

# Install OpenSSL and ca-certificates (required by tokio-postgres)
RUN apk add --no-cache openssl ca-certificates

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the application's executable from the builder stage
COPY --from=builder /usr/src/app/backend/target/release/meal_website ./

# Expose the application's port
EXPOSE  8080

# Run the application
CMD ["./meal_website"]