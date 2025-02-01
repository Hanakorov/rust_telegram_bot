use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
enum Command {
    #[command(description = "Старт.")]
    Start,
    #[command(description = "Показать справку по командам.")]
    Help,
    #[command(description = "Вычислить логарифм. Пример: /log 2 8")]
    Log,
    #[command(description = "Кубик")]
    Dice
}

#[tokio::main]
async fn main() {
    // Получаем токен из переменной окружения TELOXIDE_TOKEN
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            match Command::parse(text, "bot_username") {
                Ok(command) => {
                    match command {
                        Command::Start => {
                            bot.send_message(
                                msg.chat.id,
                                "Используй /help если хочешь что-то спросить.",
                            )
                            .await?;
                        }
                        Command::Help => {
                            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                                .await?;
                        }
                        Command::Log => {
                            let parts: Vec<&str> = text.split_whitespace().collect();
                            if parts.len() == 3 {
                                if let (Ok(base), Ok(arg)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                                    if base > 0.0 && base != 1.0 && arg > 0.0 {
                                        let result = arg.log(base);
                                        bot.send_message(
                                            msg.chat.id,
                                            format!(
                                                "Логарифм числа {} по основанию {} равен {:.5}",
                                                arg, base, result
                                            ),
                                        )
                                        .await?;
                                    } else {
                                        bot.send_message(
                                            msg.chat.id,
                                            "Основание логарифма должно быть больше 0, не равно 1, а аргумент — положительным числом.",
                                        )
                                        .await?;
                                    }
                                } else {
                                    bot.send_message(
                                        msg.chat.id,
                                        "Введите два числа: основание и аргумент логарифма. Пример: /log 2 8",
                                    )
                                    .await?;
                                }
                            } else {
                                bot.send_message(
                                    msg.chat.id,
                                    "Введите два числа: основание и аргумент логарифма. Пример: /log 2 8",
                                )
                                .await?;
                            }
                        }
                        Command::Dice => {
                            bot.send_dice(msg.chat.id).await?;
                        }
                    }
                }
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        "Неизвестная команда. Используй /help для списка доступных команд.",
                    )
                    .await?;
                }
            }
        }
        Ok(())
    })
    .await;
}