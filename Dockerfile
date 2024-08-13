# Stage 1: Build the application
FROM rust:alpine3.18 as builder

# Install necessary packages and dependencies
RUN apk add --no-cache musl-dev openssl-dev build-base curl

# Create a new directory for the project
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final lightweight image
FROM alpine:3.18

# Install any necessary runtime dependencies
RUN apk add --no-cache libgcc libstdc++ ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/selflearn /usr/local/bin/selflearn

# Expose the port that the application will run on
EXPOSE 8000

# Set the environment variable for the port (Render uses the "PORT" environment variable)
ENV ROCKET_ADDRESS=0.0.0.0

# Start the application
CMD ["selflearn"]
