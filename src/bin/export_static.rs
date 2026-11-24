use moviebox_tui::{providers::models::{CatalogItem, MediaType, ProviderKind}, service::MovieBoxService};
use serde_json::{json, Value};
use std::{collections::{BTreeMap, HashSet}, env, fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};

fn item_id(item: &CatalogItem) -> String { format!("{}:{}", item.id.provider.cache_key(), item.id.value) }
fn year_num(value: &Option<String>) -> Option<i64> { value.as_ref().and_then(|v| v.get(..4)).and_then(|v| v.parse().ok()) }
fn write_json(path: &Path, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("dist"));
    let v1 = output.join("v1");
    fs::create_dir_all(&v1)?;
    let service = MovieBoxService::new();
    let mut all = Vec::<CatalogItem>::new();
    let mut seen = HashSet::new();
    let mut trending_scores = BTreeMap::<String, f64>::new();

    // Tab 2 is the existing MovieBox browse entry used by this project's TUI.
    // Stop on the first empty page; cap requests to avoid an unbounded Actions run.
    for page in 1..=5 {
        let (items, metrics) = service.homepage("2", page).await
            .map_err(|e| format!("MovieBox homepage page {page} failed: {e}"))?;
        if items.is_empty() { break; }
        for (id, metric) in metrics {
            if let Some(score) = metric.trending.or(metric.popularity) { trending_scores.insert(id, score); }
        }
        for item in items {
            if seen.insert(item_id(&item)) { all.push(item); }
        }
    }
    if all.is_empty() { return Err("Provider returned an empty catalog; refusing to publish and preserving the previous Pages deployment.".into()); }

    let generated = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let mut enriched = Vec::<Value>::new();
    let mut genres = BTreeMap::<String, Vec<String>>::new();
    for item in &all {
        let mut row = json!({
            "id": item.id.value.clone(),
            "title": item.title.clone(),
            "name": item.title.clone(),
            "year": item.year.as_ref().and_then(|y| year_num(&item.year)),
            "releaseDate": item.year.clone(),
            "poster": item.poster_url.clone(),
            "backdrop": null,
            "thumbnail": item.poster_url.clone(),
            "description": null,
            "rating": null,
            "genre": [],
            "language": null,
            "type": if item.media_type == MediaType::Series { "series" } else { "movie" },
            "duration": null,
            "status": null,
            "detailUrl": null,
            "source": item.id.provider.cache_key(),
            "updatedAt": generated
        });
        if let Ok(details) = service.details_typed(item.id.provider, &item.id.value).await {
            let detail_type = if details.is_series() { "series" } else { "movie" };
            row["description"] = details.description.map(Value::String).unwrap_or(Value::Null);
            row["rating"] = details.imdb_rating.map(Value::String).unwrap_or(Value::Null);
            row["duration"] = details.duration.map(Value::String).unwrap_or(Value::Null);
            row["genre"] = json!(details.genres.clone());
            for genre in details.genres { genres.entry(genre).or_default().push(item.id.value.clone()); }
            row["type"] = json!(detail_type);
        }
        enriched.push(row);
    }
    if enriched.is_empty() { return Err("No serializable catalog entries were produced; refusing to publish.".into()); }

    let movies: Vec<Value> = enriched.iter().filter(|v| v["type"] == "movie").cloned().collect();
    let series: Vec<Value> = enriched.iter().filter(|v| v["type"] == "series").cloned().collect();
    let mut trending = enriched.clone();
    trending.sort_by(|a,b| {
        let aid = a["id"].as_str().unwrap_or_default();
        let bid = b["id"].as_str().unwrap_or_default();
        trending_scores.get(bid).unwrap_or(&0.0).partial_cmp(trending_scores.get(aid).unwrap_or(&0.0)).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut latest = enriched.clone();
    latest.sort_by_key(|v| std::cmp::Reverse(v["year"].as_i64().unwrap_or(0)));

    let categories: Vec<Value> = genres.into_iter().map(|(name, ids)| json!({"name":name,"count":ids.len(),"movieIds":ids})).collect();
    let catalog = json!({"version":"1.0","generatedAt":generated,"provider":"MovieBox","count":enriched.len(),"results":enriched});
    let endpoints = ["manifest.json","catalog.json","trending.json","latest.json","movies.json","series.json","categories.json"];
    let manifest = json!({"apiVersion":"1.0","generatedAt":generated,"provider":"MovieBox","count":all.len(),"endpoints":endpoints.iter().map(|e| format!("v1/{e}")).collect::<Vec<_>>(),"notes":["Static catalog generated from the existing MovieBox provider.","Search and filtering should be performed locally by clients.","Only metadata returned by the provider is exported; unavailable fields are null or omitted."]});

    write_json(&v1.join("catalog.json"), &catalog)?;
    write_json(&v1.join("manifest.json"), &manifest)?;
    write_json(&v1.join("trending.json"), &json!({"count":trending.len(),"results":trending}))?;
    write_json(&v1.join("latest.json"), &json!({"count":latest.len(),"results":latest}))?;
    write_json(&v1.join("movies.json"), &json!({"count":movies.len(),"results":movies}))?;
    write_json(&v1.join("series.json"), &json!({"count":series.len(),"results":series}))?;
    write_json(&v1.join("categories.json"), &json!({"count":categories.len(),"results":categories}))?;

    let index = r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>MovieBox Static API</title><style>body{font:16px system-ui;max-width:760px;margin:48px auto;padding:0 20px;background:#101114;color:#eee}a{color:#9db7ff}code{color:#c9d5ff}</style></head><body><h1>MovieBox Static API</h1><p>Static metadata catalog hosted on GitHub Pages. Search and filtering are client-side.</p><ul><li><a href="v1/manifest.json">Manifest</a></li><li><a href="v1/catalog.json">Catalog</a></li><li><a href="v1/trending.json">Trending order</a></li><li><a href="v1/latest.json">Latest by available year</a></li><li><a href="v1/movies.json">Movies</a></li><li><a href="v1/series.json">Series</a></li><li><a href="v1/categories.json">Categories</a></li></ul></body></html>"#;
    fs::write(output.join("index.html"), index)?;
    println!("Exported {} unique catalog items to {}", all.len(), output.display());
    Ok(())
}
