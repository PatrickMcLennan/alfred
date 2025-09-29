use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct BelongsToCollection {
    pub id: u32,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCompany {
    pub id: u32,
    pub name: String,
    pub logo_path: Option<String>,
    pub origin_country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpokenLanguage {
    pub english_name: Option<String>,
    pub iso_639_1: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

/// Full response type for `/movie/{movie_id}` (TMDB v3)
#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbMovie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: Option<u64>, // might be zero or unknown
    pub genres: Vec<Genre>,
    pub homepage: Option<String>,
    pub id: u32,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: Option<String>,
    pub revenue: Option<u64>,
    pub runtime: Option<u32>,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: String,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<u32>,
}

pub async fn get_movie_by_id(client: &Client, movie_id: u32) -> Result<TmdbMovie> {
    let api_key = env::var("TMDB_API_KEY")?;
    let mut url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}",
        movie_id, api_key
    );

    let resp = client.get(url).send().await?.error_for_status()?;
    let movie: TmdbMovie = resp.json().await?;
    Ok(movie)
}
