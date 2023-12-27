# Use Rust Nightly as the base image
FROM rustlang/rust:nightly

# Set the working directory inside the container
WORKDIR /usr/src/myapp


# Copy the current directory contents into the container
COPY . .


# Build the application
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

# Define the command to run the application
CMD ["./target/release/back4app-rust-deployment"]

