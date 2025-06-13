# Run and deploy your MCP Maps and MongoDB application

This project is designed to provide an interactive mapping experience using Google Maps and MongoDB. It leverages AI capabilities to answer geo-queries and display results on maps.

## Run Locally

**Prerequisites:** Node.js

1. Install dependencies:
   `npm install`
2. Set the `GEMINI_API_KEY` in [.env.local](.env.local) to your Gemini API key.
3. Run the app:
   `npm run dev`

## Docker Instructions

To build and run the application using Docker:

1. Build the Docker image:
   `docker build -t mcp-maps-mongodb .`
2. Run the Docker container:
   `docker run -p 3000:3000 mcp-maps-mongodb`

To deploy your app to Google Cloud Run after setting up the local development environment, here are the steps:

## 4. Prepare for Cloud Run Deployment

First, you'll need to containerize your application. Create a `Dockerfile` in your project root:

```dockerfile
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]
```

## 5. Set up Google Cloud

Install and configure the Google Cloud CLI:
```bash
# Install gcloud CLI (if not already installed)
curl https://sdk.cloud.google.com | bash

# Initialize and authenticate
gcloud init
gcloud auth login

# Set your project ID
gcloud config set project YOUR_PROJECT_ID
```

## 6. Deploy to Cloud Run

You have two main deployment options:

### Option A: Deploy from source (recommended for Next.js)
```bash
# Deploy directly from your source code
gcloud run deploy your-app-name \
  --source . \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars GEMINI_API_KEY=your_actual_api_key
```

### Option B: Build and deploy container image
```bash
# Build and push to Google Container Registry
gcloud builds submit --tag gcr.io/YOUR_PROJECT_ID/your-app-name

# Deploy the container
gcloud run deploy your-app-name \
  --image gcr.io/YOUR_PROJECT_ID/your-app-name \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars GEMINI_API_KEY=your_actual_api_key
```

## 7. Configure Environment Variables (Alternative)

For better security, you can use Google Secret Manager instead of passing the API key directly:

```bash
# Store your API key in Secret Manager
echo "your_actual_gemini_api_key" | gcloud secrets create gemini-api-key --data-file=-

# Deploy with secret reference
gcloud run deploy your-app-name \
  --source . \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-secrets GEMINI_API_KEY=gemini-api-key:latest
```

## 8. Access Your Deployed App

After deployment, Cloud Run will provide you with a URL like:
`https://your-app-name-[hash]-uc.a.run.app`

The deployment process typically takes 2-5 minutes, and your app will be automatically scaled based on traffic.

Would you like me to help you with any specific part of this deployment process or troubleshoot any issues you encounter?

Visit `http://localhost:3000` in your browser to access the application.