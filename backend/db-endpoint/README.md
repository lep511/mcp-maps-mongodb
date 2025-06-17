## How to deploy a Rust using Cloud Run

### Prerequisites
1. Install the Google Cloud SDK
2. Set up authentication: `gcloud auth login`
3. Set your project: `gcloud config set project YOUR_PROJECT_ID`
4. Enable required APIs:
   ```bash
   gcloud services enable cloudbuild.googleapis.com
   gcloud services enable run.googleapis.com
   gcloud services enable apigateway.googleapis.com
   ```
5. Rust toolchain installed (using `rustup`)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
6. Install Terraform (if using the Terraform option)

### Build with Cargo
```bash
cargo build --release
```

### Deploy in Cloud Run
```bash
# Make the deploy script executable and run it
chmod +x deploy.sh
./deploy.sh YOUR_PROJECT_ID us-central1
```

## Key Features

The API Gateway includes:

- **Request Proxying**: Routes requests to different backend services based on query parameters
- **Health Checks**: Built-in health endpoint for monitoring
- **Logging**: Request/response logging with duration tracking
- **CORS Support**: Cross-origin resource sharing enabled
- **Error Handling**: Proper HTTP status codes and error responses
- **Timeout Management**: Configurable timeouts per service
- **Header Forwarding**: Intelligent header forwarding to backend services

## Configuration

Environment variables you can set:
- `PORT`: Server port (default: 8080)
- `AUTH_SERVICE_URL`: Authentication service URL
- `USERS_SERVICE_URL`: Users service URL
- `RUST_LOG`: Logging level (info, debug, error)

The gateway routes requests like:
- `GET /api/users?service=users` → forwards to users service
- `POST /api/auth/login?service=auth` → forwards to auth service
- `GET /health` → returns gateway health status
