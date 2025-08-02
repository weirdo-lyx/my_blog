# Dockerfile for the Rust backend

# Use the official Rust image as a builder
FROM rustlang/rust:nightly as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the backend source code into the container
COPY ./backend .

# Build the application in release mode
RUN cargo build --release

# --- Final Stage ---

# Use a slim Debian image for a smaller final image size
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Expose the port the application will run on
# This should match the PORT environment variable
EXPOSE 10000

# Set the command to run the application
CMD ["backend"]
