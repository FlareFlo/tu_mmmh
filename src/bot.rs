use serenity::async_trait;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::{Embed, Message, MessageReference};
use serenity::model::gateway::Ready;
use serenity::model::id::EmojiId;
use serenity::model::mention::Mention::Emoji;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::get_menu;

#[group]
#[commands(ping)]
struct Util;

#[group]
#[commands(pizza)]
struct Food;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

pub async fn bot(token: &str) {
	let framework = StandardFramework::new()
		.configure(|c|
			c.prefix("!")
		).group(&UTIL_GROUP)
		.group(&FOOD_GROUP);

	let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Error creating client");


	if let Err(why) = client.start().await {
		println!("An error occurred while running the client: {:?} {why}", why);
	}
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
	msg.reply(ctx, "Pong!").await?;

	Ok(())
}

#[command]
async fn pizza(ctx: &Context, msg: &Message) -> CommandResult {
	let client = reqwest::ClientBuilder::new()
		.build().unwrap();

	let menu = get_menu(&client).await;
	let now = chrono::offset::Local::now().naive_local();

	let mut foods = vec![];
	for a in menu.as_array().unwrap() {
		let meal = a.pointer("/lane/name").unwrap().as_str().unwrap();
		if meal.contains("Pizza") {
			foods.push(a.get("name").unwrap().as_str().unwrap());
		}
	}

	let embeds = vec![];

	for food in foods {

	}


	msg.channel_id.send_message(ctx, |m| {
		m.content("")
		 .set_embeds(embeds)
		 .reference_message(msg)
	}).await.unwrap();

	Ok(())
}