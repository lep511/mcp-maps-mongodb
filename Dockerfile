FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

EXPOSE 3000

# Set environment variables for Google Cloud Run
ENV PORT=3000

# Health check (optional but recommended for Google Cloud Run)
# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#   CMD curl -f http://localhost:8080/ || exit 1

CMD ["npm", "start"]