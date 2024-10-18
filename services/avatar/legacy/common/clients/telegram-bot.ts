import { Bot } from "grammy";
import { TELEGRAM_BOT_API_TOKEN } from "../config.ts";

export const TelegramBot = new Bot(TELEGRAM_BOT_API_TOKEN);

TelegramBot.command(
	"start",
	(ctx) => void ctx.reply(`Channel ${ctx.chat.id} initialized.`),
);
