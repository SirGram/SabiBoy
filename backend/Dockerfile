# Step 1: Build the NestJS app
FROM node:18-alpine AS builder

WORKDIR /app

# Install dependencies
COPY backend/package*.json ./ 
RUN npm install

# Copy all the backend files and the games folder
COPY backend/. ./ 
COPY games ./games 

# Build the NestJS app
RUN npm run build

# Step 2: Run the production server
FROM node:18-alpine

WORKDIR /app

# Copy compiled code from the builder stage
COPY --from=builder /app/dist ./dist

# Copy package.json again to ensure production dependencies are available
COPY backend/package*.json ./

# Install only production dependencies
RUN npm install --production

# Expose the application port
EXPOSE 3000

# Start the app
CMD ["node", "dist/main"]
