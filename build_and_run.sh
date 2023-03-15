# Use a Rust base image
FROM rust:latest

# Create a new directory for the application
RUN mkdir /app
WORKDIR /app

# Copy the Rust project files to the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Expose the application port
EXPOSE 8000

# Run the application when the container starts
CMD ["./target/release/my_app"]
