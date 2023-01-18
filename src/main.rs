use std::str::FromStr;
use chrono::Datelike;

pub const BASE: &str = "https://sls.api.stw-on.de/v1";

pub const ID_360: usize = 111;

#[tokio::main]
async fn main() {
	let client = reqwest::ClientBuilder::new()
		.build().unwrap();
	
	let response = client.get(format_today()).send().await.unwrap();
	let res_text = response.text().await.unwrap();
	let parsed = serde_json::value::Value::from_str(&res_text).unwrap();
	let menu = parsed.pointer("/meals").unwrap();

	for a in menu.as_array().unwrap() {
		let meal = a.get("name").unwrap().as_str().unwrap();
		if meal.contains("Pizza") {
			println!("{}", meal);
		}
	}

	let serenity = serenity::client::ClientBuilder::new()
}

pub fn format_today() -> String {
	let now_iso = chrono::offset::Local::now().format("%Y-%m-%d");
	format!("{BASE}/locations/{ID_360}/menu/{now_iso}")
}
