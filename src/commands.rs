use std::error::Error;

use rand::{thread_rng, Rng};
use teloxide::payloads::GetChat;
use teloxide::utils::markdown::{bold, escape, link};
use teloxide::{prelude::*, types::InputFile, types::ParseMode, utils::command::BotCommands};

mod info;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Commands", parse_with = "split")]
pub enum Command {
    #[command(description = "Roll dice")]
    Dice,
    #[command(description = "Display an User information")]
    Who,
    #[command(description = "Display chat information")]
    Where,
}

pub async fn answer(
    b: AutoSend<Bot>,
    m: Message,
    c: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match c {
        Command::Dice => b.send_dice(m.chat.id).await?,
        Command::Who => match info::info(&m) {
            Some(s) => {
                b.send_message(m.chat.id, &s)
                    .reply_to_message_id(m.id)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?
            }
            None => {
                b.send_message(m.chat.id, "ユーザーが存在しません")
                    .reply_to_message_id(m.id)
                    .await?
            }
        },
        Command::Where => {
            let mut s: String = String::new();

            if let Some(t) = m.chat.title() {
                s = format!("{}\n", bold(&escape(&t)))
            }

            let stat = if m.chat.is_private() {
                "プライベート"
            } else {
                "パブリック"
            };

            let form = if m.chat.is_group() {
                "グループ"
            } else if m.chat.is_supergroup() {
                "スーパーグループ"
            } else if m.chat.is_channel() {
                "チャンネル"
            } else if m.chat.is_chat() {
                "個別"
            } else {
                unreachable!()
            };

            let auto_delete = match m.chat.message_auto_delete_time {
                Some(time) => time.to_string(),
                None => String::from("無効"),
            };

            s.push_str(&format!(
                "▫️状態: {}\n\
                ▫️形態: {}\n\
                ▫️ChatID: {}\n\
                ▫️メッセージ自動削除: {}",
                stat,
                form,
                escape(&m.chat.id.to_string()),
                escape(&auto_delete),
            ));

            b.send_message(m.chat.id, &s)
                .reply_to_message_id(m.id)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
    };

    Ok(())
}
