# Stage 1: Download and build Rust and Cargo
FROM rust:latest as rust_builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock to cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./

# Build a dummy Rust file to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the rest of the Rust project
COPY . .

# Build the actual Rust project
RUN cargo build --release

# Stage 2: Final image with Node.js and Rust
FROM node:14

# Set the working directory
WORKDIR /usr/src/app

# Copy the Node.js application files
COPY ./package*.json ./
COPY ./dist ./dist

# Copy the Rust build files
COPY --from=rust_builder /usr/src/app/target/release/your-rust-executable ./target/release/

# Expose the port
EXPOSE 9876

# Install application dependencies
RUN npm install

# Command to run your Node.js application
CMD ["node", "./dist/server.js"]
