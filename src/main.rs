use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            if text == "/dice" {
                bot.send_dice(msg.chat.id).await?;
            }
            if text == "/doc" {
                bot.send_message(msg.chat.id, "https://doc.rust-lang.org/book/title-page.html").await?;
            }
        }
        Ok(())
    })
    .await;
}