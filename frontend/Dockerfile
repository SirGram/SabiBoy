# ========================
# Stage 1: Build the WASM Module
# ========================
FROM node:20-alpine as wasm-builder
WORKDIR /wasm-build

# Install required dependencies
RUN apk add --no-cache curl gcc musl-dev make

# Install Rust and Cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack globally
RUN cargo install wasm-pack

# Add the wasm32 target for Rust
RUN rustup target add wasm32-unknown-unknown

# Copy the necessary emulator files
COPY emulator/wasm /wasm-build/emulator/wasm
COPY emulator/core /wasm-build/emulator/core

# Move to the wasm directory and verify files exist
WORKDIR /wasm-build/emulator/wasm
RUN ls -la

# Build the WASM module
RUN wasm-pack build --out-dir pkg

# Verify that pkg/ was created
RUN ls -la pkg

# ========================
# Stage 2: Build the Frontend Application
# ========================
FROM node:20-alpine as builder
WORKDIR /frontend

# Copy package files and install dependencies
COPY frontend/package*.json ./
RUN npm install

# Ensure WASM directory exists
RUN mkdir -p src/wasm/pkg

# Copy the built WASM module from wasm-builder stage
COPY --from=wasm-builder /wasm-build/emulator/wasm/pkg /frontend/src/wasm/pkg/

# Copy frontend source files and build the app
COPY frontend/ ./
RUN npm run build

# ========================
# Stage 3: Production Stage with Nginx
# ========================
FROM nginx:stable-alpine

# Remove default Nginx static assets
RUN rm -rf /usr/share/nginx/html/*

# Copy built frontend app from builder stage
COPY --from=builder /frontend/dist /usr/share/nginx/html

# Optionally, copy custom Nginx configuration
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80
EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
