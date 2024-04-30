# Use a base image with the latest version of Rust installed
FROM rust:latest 

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Add windows dependencies
RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64 

# Add the target and toolchain
RUN rustup target add x86_64-pc-windows-gnu 
RUN for i in 1 2 3 4 5; do \
    rustup toolchain install stable-x86_64-pc-windows-gnu && break || sleep 1; \
    done

# Specify the command to run when the container starts
CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"]