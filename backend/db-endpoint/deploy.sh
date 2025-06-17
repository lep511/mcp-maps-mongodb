#!/bin/bash

set -e

PROJECT_ID=${1:-your-project-id}
REGION=${2:-us-central1}
SERVICE_NAME="db-endpoint"

if ! gcloud projects describe "$PROJECT_ID" &> /dev/null; then
    echo "Project ID '$PROJECT_ID' does not exist or you do not have access to it."
    exit 1
fi

echo "Deploying API Gateway to project: $PROJECT_ID"

# Check if Artifact exists
if gcloud artifacts repositories describe $SERVICE_NAME-repo --location=$REGION --project=$PROJECT_ID &> /dev/null; then
    echo "Artifact Registry repository already exists."
else
    echo "Creating Artifact Registry repository..."
    # Create Artifact Registry repository
    gcloud artifacts repositories create $SERVICE_NAME-repo \
        --repository-format=docker \
        --location=$REGION
fi

# Configure Docker authentication
gcloud auth configure-docker $REGION-docker.pkg.dev

# Build and tag image
docker build -t $REGION-docker.pkg.dev/$PROJECT_ID/$SERVICE_NAME-repo/$SERVICE_NAME-app:latest .

# Push to registry
docker push $REGION-docker.pkg.dev/$PROJECT_ID/$SERVICE_NAME-repo/$SERVICE_NAME-app:latest

# Build and deploy the Rust service to Cloud Run
# echo "Building and deploying Rust service..."
# gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME

if [ -z "$MONGODB_URI" ]; then
    echo "Error: MONGODB_URI environment variable is not set."
    exit 1
fi

gcloud run deploy $SERVICE_NAME \
    --image $REGION-docker.pkg.dev/$PROJECT_ID/$SERVICE_NAME-repo/$SERVICE_NAME-app:latest \
    --platform managed \
    --region $REGION \
    --allow-unauthenticated \
    --memory 512Mi \
    --timeout=90 \
    --concurrency=100 \
    --set-env-vars="MONGODB_URI=$MONGODB_URI" \
    --cpu 1 \
    --max-instances 10 \
    --min-instances 1
    
# Get the Cloud Run service URL
SERVICE_URL=$(gcloud run services describe $SERVICE_NAME \
    --platform managed \
    --region $REGION \
    --format 'value(status.url)')

echo "Cloud Run service deployed at: $SERVICE_URL"