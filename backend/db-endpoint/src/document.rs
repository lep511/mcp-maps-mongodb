use mongodb::bson::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Listing {
    #[serde(rename = "_id")]
    pub id: i32,
    pub listing_url: String,
    pub name: String,
    // Fields that might be missing are now wrapped in Option
    pub summary: Option<String>,
    pub space: Option<String>,
    pub description: String,
    pub neighborhood_overview: Option<String>,
    pub notes: Option<String>,
    pub transit: Option<String>,
    pub access: Option<String>,
    pub interaction: Option<String>,
    pub house_rules: Option<String>,
    pub property_type: Option<String>, // property_type is missing in the new JSON
    pub room_type: String,
    pub bed_type: String,
    pub minimum_nights: i32,
    pub maximum_nights: i32,
    pub cancellation_policy: Option<String>,
    pub last_scraped: Option<DateTime>,
    pub calendar_last_scraped: Option<DateTime>,
    pub first_review: Option<DateTime>,
    pub last_review: Option<DateTime>,
    pub accommodates: i32,
    pub bedrooms: i32,
    pub beds: i32,
    pub number_of_reviews: i32,
    // Use Option for f64 in case the field is missing or not a number
    #[serde(default)] // If bathrooms is missing, it will be the default (0.0)
    pub bathrooms: f64,
    pub amenities: Vec<String>,
    pub price: f64,
    pub security_deposit: Option<i32>,
    pub cleaning_fee: Option<i32>,
    pub extra_people: i32,
    pub guests_included: i32,
    // New optional fields for weekly and monthly prices
    pub weekly_price: Option<f64>,
    pub monthly_price: Option<f64>,
    pub host: Host,
    pub address: Address,
    pub availability: Availability,
    pub review_scores: ReviewScores,
    // Reviews might not be present in all documents
    #[serde(default)] // If reviews is missing, it will be an empty Vec
    pub reviews: Vec<Review>,
    // New field for text embeddings
    #[serde(default)]
    pub text_embeddings: Vec<f64>,
}


#[derive(Debug, Deserialize)]
pub struct Host {
    pub host_id: String,
    pub host_url: String,
    pub host_name: String,
    pub host_location: String,
    pub host_about: Option<String>,
    pub host_response_time: Option<String>,
    pub host_thumbnail_url: String,
    pub host_picture_url: String,
    pub host_neighbourhood: Option<String>,
    pub host_response_rate: Option<i32>,
    pub host_is_superhost: bool,
    pub host_has_profile_pic: bool,
    pub host_identity_verified: bool,
    pub host_listings_count: i32,
    pub host_total_listings_count: i32,
    pub host_verifications: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub street: String,
    pub suburb: Option<String>,
    pub government_area: String,
    pub market: String,
    pub country: String,
    pub country_code: String,
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    #[serde(rename = "type")]
    pub location_type: String,
    pub coordinates: Vec<f64>,
    pub is_location_exact: bool,
}

#[derive(Debug, Deserialize)]
pub struct Availability {
    pub availability_30: i32,
    pub availability_60: i32,
    pub availability_90: i32,
    pub availability_365: i32,
}

#[derive(Debug, Deserialize)]
pub struct ReviewScores {
    pub review_scores_accuracy: Option<i32>,
    pub review_scores_cleanliness: Option<i32>,
    pub review_scores_checkin: Option<i32>,
    pub review_scores_communication: Option<i32>,
    pub review_scores_location: Option<i32>,
    pub review_scores_value: Option<i32>,
    pub review_scores_rating: Option<i32>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Review {
    #[serde(rename = "_id")]
    pub id: String,
    pub date: Option<DateTime>,
    pub listing_id: String,
    pub reviewer_id: String,
    pub reviewer_name: String,
    pub comments: Option<String>,
}