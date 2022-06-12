use std::env;
use teloxide::dptree::endpoint;
use teloxide::prelude::*;

mod commands;
mod youtube;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Evil-Maid bot...");

    let bot = Bot::new(env::var("EVILMAID_TOKEN").unwrap()).auto_send();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<commands::Command>()
                .endpoint(commands::answer),
        )
        .branch(
            dptree::filter(|msg: Message| msg.text().is_some())
                // .branch(filter(|| commands::roll_dice())).endpoint(commands::res_quote),
                .branch(dptree::filter(|m: Message| youtube::contains_yt(m)))
                .endpoint(youtube::answer),
        );

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}
