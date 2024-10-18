import { load } from "std/dotenv/mod.ts";
import defaultSettings from "/.config/avatar.json" with { type: "json" };

export const { GROQ_API_TOKEN, TELEGRAM_BOT_API_TOKEN } = await load();

export const {
	responses: RESPONSE_PRESET,
	personality: PERSONALITY_PRESET,
	platforms: { telegram: TELEGRAM_SETTINGS },
} = defaultSettings;

export const REPLY_TRIGGERS = [
	`${PERSONALITY_PRESET.metadata.name}2,`,
	`@${TELEGRAM_SETTINGS.username}`,
];
