use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use mongodb::{bson::doc, Client, Collection};
use openai::embed::EmbedOpenAI;
use env_logger::Env;
use futures::TryStreamExt;

mod document;
use document::{ShortTermRental, ResponseSearch};

// OpenAI
pub mod openai;
pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;

#[derive(Debug, Clone)]
struct AppState {
    http_client: reqwest::Client,
    services: HashMap<String, ServiceConfig>,
    collection: Collection<ResponseSearch>,
}
#[derive(Debug, Clone)]
struct ServiceConfig {
    base_url: String,
    timeout_ms: u64,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    service: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    embed: Option<Vec<f32>>,
    error: Option<String>,
    request_id: String,
}

#[derive(Debug, Serialize, Clone)]
struct HealthCheck {
    status: String,
    timestamp: String,
    version: String,
}

async fn get_data(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<ResponseSearch>>, StatusCode> {
    
    // let limit = params.get("limit")
    //     .and_then(|l| l.parse::<i64>().ok())
    //     .unwrap_or(10);
    
    let results = state.collection
        .find_one(doc! { "name": "Private Room in Bushwick" })
        .await
        .expect("Failed to query database");

    Ok(Json(ApiResponse {
        success: true,
        data: results,
        embed: None,
        error: None,
        request_id: uuid::Uuid::new_v4().to_string(),
    }))
}

// async fn store_data(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<serde_json::Value>,
// ) -> Result<Json<ApiResponse<DataEntry>>, StatusCode> {
//     let collection: Collection<DataEntry> = state.db.collection("data_entries");
    
//     let data_entry = DataEntry {
//         _id: None,
//         data: payload,
//         timestamp: chrono::Utc::now().to_rfc3339(),
//         source: "api_gateway".to_string(),
//     };

//     match collection.insert_one(&data_entry).await {
//         Ok(result) => {
//             let mut stored_entry = data_entry;
//             stored_entry._id = result.inserted_id.as_object_id();
            
//             Ok(Json(ApiResponse {
//                 success: true,
//                 data: Some(stored_entry),
//                 error: None,
//                 request_id: uuid::Uuid::new_v4().to_string(),
//             }))
//         }
//         Err(e) => {
//             tracing::error!("Failed to store data: {}", e);
//             Err(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//     }
// }

async fn mock_get_data(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<ResponseSearch>>, StatusCode> {

    let mock_result = state.collection
        .find_one(doc! { "_id": 10084023 })
        .await
        .expect("Failed to query database");

    // let mock_data = ShortTermRental {
    //     id: 123,
    //     name: "Beautiful Loft".to_string(),
    //     summary: Some("A nice and cozy place".to_string()),
    //     ..Default::default() // El resto se rellena con los valores por defecto
    // };

    Ok(Json(ApiResponse {
        success: true,
        data: mock_result,
        embed: None,
        error: None,
        request_id: uuid::Uuid::new_v4().to_string(),
    }))
}

async fn post_embed(
    Path(path): Path<String>,
    Query(params): Query<QueryParams>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let service_name = params.service.unwrap_or_else(|| "default".to_string());

    let llm = EmbedOpenAI::new("text-embedding-3-small");   

    let input_str = body.trim().to_string();

    if input_str.is_empty() {
        tracing::error!("Input string is empty");
        return Err(StatusCode::BAD_REQUEST);
    }
    tracing::info!("Embedding: {}", input_str);

    let response = llm
        .with_dimensions(1536)
        .embed_content(&input_str)
        .await
        .expect("Failed to get embedding");

    let embeddings: Vec<f32> = response.data[0].embedding.clone();
        
    let pipeline = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": embeddings,
            "path": "text_embeddings",
            "numCandidates": 120,
            "index": "vector_index",
            "limit": 1
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "name": 1,
                "summary": 1,
                "description": 1,
                "beds": 1,
                "bathrooms": 1,
                "bedrooms": 1,
                "amenities": 1,
                "price": 1,
            }
        }
    ]; 

    let mut results = state.collection
        .aggregate(pipeline)
        .await
        .expect("Failed to execute aggregation");
    
    let first_result = results
        .try_next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Convert Document to serde_json::Value
    let bson_value = mongodb::bson::to_bson(&first_result)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let json_value = serde_json::Value::from(bson_value);

    Ok(Json(ApiResponse {
        success: true,
        data: Some(json_value),
        embed: None, // Some(embeddings),
        error: None,
        request_id: uuid::Uuid::new_v4().to_string(),
    }))
}

async fn health_check() -> Json<ApiResponse<HealthCheck>> {
    let health = HealthCheck {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(ApiResponse {
        success: true,
        data: Some(health),
        embed: None,
        error: None,
        request_id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn proxy_get_request(
    Path(path): Path<String>,
    Query(params): Query<QueryParams>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let service_name = params.service.unwrap_or_else(|| "default".to_string());
    
    let service_config = state.services.get(&service_name)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let url = format!("{}/{}", service_config.base_url, path);
    
    let mut request = state.http_client.get(&url);
    
    // Forward relevant headers
    for (name, value) in headers.iter() {
        if should_forward_header(name.as_str()) {
            request = request.header(name, value);
        }
    }

    let response = request
        .timeout(std::time::Duration::from_millis(service_config.timeout_ms))
        .send()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let json_response: serde_json::Value = response
        .json()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(json_response))
}

async fn proxy_post_request(
    Path(path): Path<String>,
    Query(params): Query<QueryParams>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let service_name = params.service.unwrap_or_else(|| "default".to_string());
    
    let service_config = state.services.get(&service_name)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let url = format!("{}/{}", service_config.base_url, path);
    
    let mut request = state.http_client.post(&url);
    
    // Forward relevant headers
    for (name, value) in headers.iter() {
        if should_forward_header(name.as_str()) {
            request = request.header(name, value);
        }
    }

    let response = request
        .body(body)
        .timeout(std::time::Duration::from_millis(service_config.timeout_ms))
        .send()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let json_response: serde_json::Value = response
        .json()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(json_response))
}

fn should_forward_header(header_name: &str) -> bool {
    match header_name.to_lowercase().as_str() {
        "authorization" | "content-type" | "accept" | "user-agent" => true,
        name if name.starts_with("x-") => true,
        _ => false,
    }
}

async fn request_logging_middleware(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(req).await;
    
    let elapsed = start.elapsed();
    tracing::info!(
        method = %method,
        uri = %uri,
        status = response.status().as_u16(),
        duration_ms = elapsed.as_millis(),
        "Request processed"
    );
    
    response
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // MongoDB connection
    let mongodb_uri = std::env::var("MONGODB_URI")
        .expect("MONGODB_URI environment variable must be set");
    
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");
        
    let collection: Collection<ResponseSearch> = client
        .database("sample_airbnb")
        .collection("airbnb");

    // Configure services
    let mut services = HashMap::new();
    services.insert("auth".to_string(), ServiceConfig {
        base_url: std::env::var("AUTH_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8001".to_string()),
        timeout_ms: 5000,
    });
    services.insert("users".to_string(), ServiceConfig {
        base_url: std::env::var("USERS_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8002".to_string()),
        timeout_ms: 3000,
    });

    let state = Arc::new(AppState {
        http_client: reqwest::Client::new(),
        services,
        collection,
    });

    let app = Router::new()
        .route("/health", get(health_check))
        // .route("/data", post(store_data))
        .route("/data", get(get_data))
        .route("/mock", get(mock_get_data))
        .route("/embed/{*path}", post(post_embed))
        .route("/api/{*path}", get(proxy_get_request))
        .route("/api/{*path}", post(proxy_post_request))
        .layer(middleware::from_fn(request_logging_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("DB-ENDPOINT listening on port {}", port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}