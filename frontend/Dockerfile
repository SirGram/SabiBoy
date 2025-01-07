# Build Stage
FROM node:20-alpine as frontend-builder

# Set working directory
WORKDIR /frontend

# Copy frontend package files
COPY frontend/package*.json ./
RUN npm install

# Copy frontend source files
COPY frontend/ ./
RUN npm run build

# Production Stage (No NGINX, directly serve with Node.js)
FROM node:20-alpine

WORKDIR /frontend

# Copy the built files from the frontend builder stage
COPY --from=frontend-builder /frontend/dist ./dist

# Install a simple HTTP server (e.g., serve)
RUN npm install -g serve

# Expose the default HTTP port
EXPOSE 8080

# Start the server
CMD ["serve", "-s", "dist", "-l", "8080"]
