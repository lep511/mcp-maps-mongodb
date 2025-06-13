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

Visit `http://localhost:3000` in your browser to access the application.