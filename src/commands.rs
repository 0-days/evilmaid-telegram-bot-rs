use std::error::Error;

use rand::{thread_rng, Rng};
use teloxide::utils::markdown::{bold, link};
use teloxide::{prelude::*, types::InputFile, types::ParseMode, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Commands", parse_with = "split")]
pub enum Command {
    #[command(description = "Roll dice")]
    Dice,
}

pub async fn answer(
    b: AutoSend<Bot>,
    m: Message,
    c: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match c {
        Command::Dice => b.send_dice(m.chat.id).await?,
    };

    Ok(())
}
