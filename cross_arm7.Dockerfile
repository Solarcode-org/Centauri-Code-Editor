# Use a base image with the latest version of Rust installed
FROM rust:latest 

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Add ARM7 dependencies
RUN apt update && apt upgrade -y 
RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross

# Add the target and toolchain
RUN rustup target add armv7-unknown-linux-gnueabihf 
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf 

# Add environment variables.
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++

# Specify the command to run when the container starts
CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf"]
