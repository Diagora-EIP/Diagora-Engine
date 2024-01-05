# Stage 1: Build Rust executable
FROM rust:latest AS rust_builder

# Set the working directory
WORKDIR /usr/src/app

# Copy only the necessary Rust files for building
COPY Cargo.lock Cargo.toml ./

# Copy the entire Rust project (excluding unnecessary files)
COPY src ./src

# Build the Rust executable
RUN cargo build --release

# Stage 2: Final image with Node.js and Rust
FROM node:14

# Set the working directory
WORKDIR /usr/src/app

# Copy the Node.js application files
COPY package*.json ./

# Copy the compiled Node.js files
COPY . ./

# Copy the Rust build files
COPY --from=rust_builder /usr/src/app/target/release/engine ./target/release/

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Expose the port
EXPOSE 9876

# Install application dependencies
RUN npm install

# Command to run your Node.js application
CMD ["node", "server.js"]