use futures::{stream, StreamExt};
use reqwest::Client;

use lib::clients::tmdb::get_movie_by_id::get_movie_by_id;
use lib::clients::tmdb::get_movie_by_id::TmdbMovie;

pub async fn fetch_movies_batch(
    client: &Client,
    ids: &[u32],
) -> (Vec<TmdbMovie>, Vec<(u32, String)>) {
    let fetches = stream::iter(ids.iter().copied().map(|id| {
        let client = client.clone();
        async move {
            match get_movie_by_id(&client, id).await {
                Ok(movie) => Ok(movie),
                Err(e) => Err((id, format!("{:#}", e))),
            }
        }
    }));

    let mut ok = Vec::new();
    let mut err = Vec::new();

    let results = fetches.buffer_unordered(12).collect::<Vec<_>>().await;

    for res in results {
        match res {
            Ok(movie) => ok.push(movie),
            Err((id, msg)) => {
                log::warn!("TMDB fetch failed for id={id}: {msg}");
                err.push((id, msg));
            }
        }
    }

    println!("ok: {:?}", ok);
    println!("err: {:?}", err);

    (ok, err)
}
