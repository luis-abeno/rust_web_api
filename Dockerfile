# Use an official Rust runtime as a parent image
FROM rust:1.75

# Set the working directory in the container to /api
WORKDIR /api

# Copy the current directory contents into the container at /api
COPY . /api

# Build the application
RUN cargo build --release

# Make port 8080 available to the world outside this container
EXPOSE 8080

# Run the application when the container launches
CMD ["cargo", "run", "--release"]