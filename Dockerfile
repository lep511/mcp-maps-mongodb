FROM node:18 AS build

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package.json package-lock.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the application
RUN npm run build

# Start a new stage for the production image
FROM node:18 AS production

# Set the working directory
WORKDIR /app

# Copy the build output from the previous stage
COPY --from=build /app/dist ./dist

# Install serve to serve the static files
RUN npm install -g serve

# Expose the port the app runs on
EXPOSE 8080

# Set environment variables for Google Cloud Run
ENV PORT=8080

# Health check (optional but recommended for Google Cloud Run)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/ || exit 1

# Command to run the application
CMD ["serve", "-s", "dist"]