FROM rust:latest as rust_builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

COPY . .

RUN cargo build --release

FROM node:19-bullseye

WORKDIR /usr/src/app

COPY ./package*.json ./

RUN npm install

COPY . .

RUN npm run build

FROM node:14

WORKDIR /usr/src/app

COPY --from=node_builder /usr/src/app/dist ./dist

COPY --from=rust_builder /usr/src/app/target/release/your-rust-executable ./target/release/

EXPOSE 9876

CMD ["node", "./dist/server.js"]
