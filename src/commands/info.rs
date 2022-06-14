use std::error::Error;
use teloxide::utils::markdown::{bold, escape, link};
use teloxide::{prelude::*, types::ParseMode};

pub fn info(m: &Message) -> Option<String> {
    if let Some(rep) = m.reply_to_message() {
        if let Some(u) = rep.from() {
            let ty = if u.is_bot { "Bot" } else { "User" };
            let mut s: String = format!(
                "{}\n\
                ▫️ID: {}\n\
                ▫️属性: {}\n",
                bold(&u.full_name()),
                u.id,
                ty,
            );
            if let Some(uname) = &u.username {
                s.push_str(&format!("▫️Username: {}\n", uname))
            }
            if let Some(lang) = &u.language_code {
                s.push_str(&format!("▫️言語: {}\n", lang))
            }
            s.push_str(&format!(
                "\n{}",
                link(u.preferably_tme_url().as_str(), "Share this user")
            ));
            return Some(s);
        }
    }
    None
}
