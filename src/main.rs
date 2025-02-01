use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // Создаем бота, который получает токен из переменной окружения TELOXIDE_TOKEN
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // Проверяем, есть ли текст в сообщении и является ли он командой "/dice"
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