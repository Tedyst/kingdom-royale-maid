use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[bucket = "stats"]
#[owners_only]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data
        .get::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name = k, amount = v);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &contents).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}