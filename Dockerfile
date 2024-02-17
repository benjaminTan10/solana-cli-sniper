# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the pre-built binary to the container
COPY . .

# Specify the command to run your application
CMD ["./firstx"]
