# Itaca Rental Location Search

A smart search tool for short-term rentals that understands natural language queries. Search by accommodation type, specific rental names, or location areas using advanced vector search capabilities.

## Overview

Itaca allows users to find short-term rentals using intuitive search queries like:
- "cozy cabin near the lake"
- "The Blue House rental"
- "apartments in downtown Portland"

The system uses vector search to understand the meaning behind your search terms, not just exact keyword matching.

## Architecture

- **Frontend**: JavaScript + Vue.js
- **Backend**: Rust with MongoDB Atlas Search
- **Deployment**: Google Cloud Run
- **Database**: MongoDB with vector search capabilities

## Project Structure

```
main/
├── frontend/          # Vue.js application
│   ├── src/
│   ├── package.json
|   ├── Dockerfile
│   └── ...
├── backend/           # Rust API server
│   ├── src/
│   ├── Cargo.toml
|   ├── Dockerfile
│   └── ...
└── README.md
```

## Getting Started

### Prerequisites

- Node.js (v18 or higher)
- Rust (latest stable)
- MongoDB Atlas account
- Google Cloud Platform account

## Features

- **Natural Language Search**: Understanding user intent beyond exact keywords
- **Vector Search**: Powered by MongoDB Atlas Search for semantic matching
- **Responsive Design**: Works on desktop and mobile devices
- **Fast Performance**: Rust backend for optimal speed
- **Scalable Infrastructure**: Google Cloud Run handles traffic automatically

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions or issues, please open a GitHub issue or contact the development team.
