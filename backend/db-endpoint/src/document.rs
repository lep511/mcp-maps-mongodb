use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortTermRental {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
    // Fields optionaly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighborhood_overview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bed_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_nights: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_nights: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scraped: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_last_scraped: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_review: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_review: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accommodates: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrooms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_reviews: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bathrooms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amenities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_deposit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleaning_fee: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_people: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_included: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores: Option<ReviewScores>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<Review>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_embeddings: Option<Vec<f64>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub street: String,
    pub suburb: Option<String>,
    pub government_area: String,
    pub market: String,
    pub country: String,
    pub country_code: String,
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    #[serde(rename = "type")]
    pub location_type: String,
    pub coordinates: Vec<f64>,
    pub is_location_exact: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Availability {
    pub availability_30: i32,
    pub availability_60: i32,
    pub availability_90: i32,
    pub availability_365: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewScores {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_accuracy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_cleanliness: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_checkin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_communication: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_location: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_scores_rating: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    #[serde(rename = "_id")]
    pub id: String,
    pub date: Option<DateTime>,
    pub listing_id: String,
    pub reviewer_id: String,
    pub reviewer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl Default for ShortTermRental {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            description: None,
            summary: None,
            space: None,
            neighborhood_overview: None,
            notes: None,
            transit: None,
            access: None,
            interaction: None,
            house_rules: None,
            property_type: None,
            room_type: None,
            bed_type: None,
            minimum_nights: None,
            maximum_nights: None,
            cancellation_policy: None,
            last_scraped: None,
            calendar_last_scraped: None,
            first_review: None,
            last_review: None,
            accommodates: None,
            bedrooms: None,
            beds: None,
            number_of_reviews: None,
            bathrooms: None,
            amenities: None,
            price: None,
            security_deposit: None,
            cleaning_fee: None,
            extra_people: None,
            guests_included: None,
            weekly_price: None,
            monthly_price: None,
            host: None,
            address: None,
            availability: None,
            review_scores: None,
            reviews: None,
            text_embeddings: None,
        }
    }
}
