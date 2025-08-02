# Dockerfile for the Vue.js frontend

# Stage 1: Build the application
FROM node:20-alpine as builder

# Set the working directory
WORKDIR /usr/src/app

# Accept the VITE_API_URL build argument
ARG VITE_API_URL

# Set the VITE_API_URL environment variable
ENV VITE_API_URL=$VITE_API_URL

# Copy package.json and package-lock.json
COPY frontend/package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the frontend source code
COPY frontend/ ./

# Build the application
RUN npm run build

# Stage 2: Serve the application with Nginx
FROM nginx:stable-alpine

# Copy the built assets from the builder stage
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
