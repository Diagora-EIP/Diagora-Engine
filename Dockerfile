# Stage 1: Build Rust executable
FROM rust:latest AS rust_builder

# Set the working directory for Rust project
WORKDIR /usr/src/app/ItinaryEngine

# Copy only the necessary Rust files for building
COPY ItinaryEngine/Cargo.lock ItinaryEngine/Cargo.toml ./

# Create a directory for Rust source code
RUN mkdir -p src

# Copy the entire Rust project (excluding unnecessary files)
COPY ItinaryEngine/src ./src

# Build the Rust executable
RUN cargo build --release

# Stage 2: Final image with Node.js and Rust
FROM node:19-bullseye

# Set the working directory
WORKDIR /usr/src/app

# Copy the Node.js application files
COPY package*.json ./

# Copy the compiled Node.js files
COPY . ./

# Copy the Rust build files
COPY --from=rust_builder /usr/src/app/ItinaryEngine/target/release/engine ./target/release/

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Python 3 and required Python packages
RUN apt-get update && \
    apt-get install -y python3 python3-pip && \
    python3 -m pip install --upgrade pip && \
    pip install schedule==1.1.0 python-dotenv==0.19.0 supabase requests

# Expose the port
EXPOSE 9876

# Install application dependencies
RUN npm install

# Command to run your Node.js application
CMD ["node", "server.js"]
