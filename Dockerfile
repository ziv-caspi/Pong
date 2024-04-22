# Use the official Node.js image as a build environment for SvelteKit
FROM node:latest AS svelte_builder

# Set working directory for the SvelteKit project
WORKDIR /app/pong-web

# Copy SvelteKit project files
COPY pong-web .

# Install dependencies and build the SvelteKit project
RUN npm install
RUN npm run build


# cargo chef stuff
FROM rust:latest AS chef
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust_builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
COPY --from=svelte_builder /app/web/dist ./web/dist
RUN cargo build --release

# # Create a new stage for Rust building
# FROM rust:latest AS rust_builder

# # Set working directory for the Rust project
# WORKDIR /app

# # Copy the Rust project files
# COPY . .
# # Build the Rust binary
# #RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --release
# # --target=x86_64-unknown-linux-musl


# Create a new stage for the final image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the built SvelteKit files from the previous stage
#COPY --from=svelte_builder /app/web/dist ./target/release/web/dist

# Copy the built Rust binary from the previous stage
COPY --from=rust_builder /app/target/release/pong ./target/release/pong

# Expose any necessary ports
EXPOSE 3030
EXPOSE 5000

# Run the Rust binary
CMD ["./target/release/pong"]
