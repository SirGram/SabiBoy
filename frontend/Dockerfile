# Build Stage
FROM node:20-alpine as builder

WORKDIR /frontend

# Copy package files and install dependencies
COPY frontend/package*.json ./
RUN npm install

# Copy source files and build the app
COPY frontend/ ./
RUN npm run build

# Production Stage with Nginx
FROM nginx:stable-alpine

# Remove default Nginx static assets
RUN rm -rf /usr/share/nginx/html/*

# Copy the built app from the builder stage into Nginx's public folder
COPY --from=builder /frontend/dist /usr/share/nginx/html

# Optionally, copy your custom Nginx configuration file
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80 (Nginx default)
EXPOSE 80

# Start Nginx in the foreground
CMD ["nginx", "-g", "daemon off;"]