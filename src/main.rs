mod bot;

use std::env::var;
use std::str::FromStr;
use chrono::{Datelike, Days, Weekday};
use reqwest::Client;
use serde_json::Value;
use crate::bot::bot;

pub const BASE: &str = "https://sls.api.stw-on.de/v1";

pub const ID_360: usize = 111;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let token = var("DISCORD_TOKEN").expect("Token missing, get it here https://discord.com/developers/applications/1065217120104873994/bot");
	let client = reqwest::ClientBuilder::new()
		.build().unwrap();

	let menu = get_menu(&client).await;

	for a in menu.as_array().unwrap() {
		let meal = a.get("name").unwrap().as_str().unwrap();
		if meal.contains("Pizza") {
			println!("{}", meal);
		}
	}

	bot(&token).await;
}

// Returns request url for next wednesday
pub fn format_today() -> String {
	let mut now = chrono::offset::Local::now();
	while now.weekday() != Weekday::Wed {
		now = now.checked_add_days(Days::new(1)).unwrap();
	}

	let iso = now.format("%Y-%m-%d");
	format!("{BASE}/locations/{ID_360}/menu/{iso}")
}

pub async fn get_menu(client: &Client) -> Value {
	let uri = format_today();
	let response = client.get(&uri).send().await.unwrap();
	let res_text = response.text().await.unwrap();
	let parsed = serde_json::value::Value::from_str(&res_text).unwrap();
	parsed.pointer("/meals").unwrap().to_owned()
}
