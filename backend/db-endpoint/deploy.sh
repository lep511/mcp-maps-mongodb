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

gcloud run deploy $SERVICE_NAME \
    --image $REGION-docker.pkg.dev/$PROJECT_ID/$SERVICE_NAME-repo/$SERVICE_NAME-app:latest \
    --platform managed \
    --region $REGION \
    --allow-unauthenticated \
    --memory 512Mi \
    --timeout=300 \
    --concurrency=100 \
    --cpu 1 \
    --max-instances 10

# Get the Cloud Run service URL
SERVICE_URL=$(gcloud run services describe $SERVICE_NAME \
    --platform managed \
    --region $REGION \
    --format 'value(status.url)')

echo "Cloud Run service deployed at: $SERVICE_URL"

# Update the API config with the actual service URL
sed "s|https://your-cloud-run-service-url|$SERVICE_URL|g" api-config.yaml > api-config-updated.yaml

# Create API Gateway
echo "Creating API Gateway..."

# Create API
gcloud api-gateway apis create rust-db-endpoint \
    --project=$PROJECT_ID || echo "API already exists"

# Create API config
gcloud api-gateway api-configs create rust-api-config \
    --api=rust-db-endpoint \
    --openapi-spec=api-config-updated.yaml \
    --project=$PROJECT_ID

# Create Gateway
gcloud api-gateway gateways create rust-gateway \
    --api=rust-db-endpoint \
    --api-config=rust-api-config \
    --location=$REGION \
    --project=$PROJECT_ID

# Get the gateway URL
GATEWAY_URL=$(gcloud api-gateway gateways describe rust-gateway \
    --location=$REGION \
    --project=$PROJECT_ID \
    --format='value(defaultHostname)')

echo "API Gateway deployed at: https://$GATEWAY_URL"