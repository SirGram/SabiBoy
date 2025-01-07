# Build stage
FROM node:18 AS builder

WORKDIR /app

# Copy package files first
COPY backend/package*.json ./
COPY backend/.env ./

# Install dependencies
RUN npm ci

# Copy source code
COPY backend/. ./

# Build the application
RUN npm run build

# Production stage
FROM node:18

WORKDIR /app

# Copy package files and install production dependencies
COPY backend/package*.json ./
COPY backend/.env ./
RUN npm ci --only=production && \
    npm rebuild bcrypt --build-from-source

# Copy built application
COPY --from=builder /app/dist ./dist

# Create games directory
RUN mkdir -p /app/games

# Copy games directory contents
COPY games/. /app/games/

# Set proper ownership and permissions
RUN chown -R node:node /app && \
    chmod -R 755 /app/games

# Verify the games directory
RUN ls -la /app/games

USER node

EXPOSE 8080

CMD ["node", "dist/main"]