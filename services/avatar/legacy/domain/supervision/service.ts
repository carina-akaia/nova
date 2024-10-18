import { createEffect, createEvent, sample } from "effector";
import { A, D } from "ts-belt";
import { Groq, TelegramBot } from "../../common/clients/mod.ts";
import { REPLY_TRIGGERS, RESPONSE_PRESET, TELEGRAM_SETTINGS } from "../../common/config.ts";
import { TextResponseExpected } from "../../common/lib/mod.ts";
import { $systemInstructions } from "./model.ts";

const { allies } = TELEGRAM_SETTINGS;

export const textMessageReceived = createEvent<TextResponseExpected>("Message received");

const textResponseFx = createEffect("Text response", {
	handler: async (
		{ query, reply }: TextResponseExpected,
	) =>
		reply(
			await Groq.handle({
				temperature: 1.2,

				messages: [
					{ role: "system", content: $systemInstructions.getState() },
					{ role: "user", content: query },
				],
			}),
		),
});

sample({
	source: textMessageReceived,
	target: textResponseFx,
});

TelegramBot.on("message", (ctx) => {
	if (ctx.message.text === "fun") void ctx.react("😁");
	if (ctx.message.text === "love") void ctx.react("❤");

	if (typeof ctx.message?.text === "string") {
		const isContainingMention = A.any(
			REPLY_TRIGGERS,
			(trigger) => ctx.message.text?.startsWith(trigger) ?? false,
		);

		const isFromAlly = allies.includes(ctx.senderChat?.id ?? ctx.msg.from.id);

		if (isContainingMention) {
			console.log(ctx.message.text);

			if (isFromAlly) {
				void textMessageReceived({
					query: ctx.message.text,

					reply: (response: string) =>
						ctx.reply(response, { reply_parameters: D.selectKeys(ctx.msg, ["message_id"]) }),
				});
			} else {
				void ctx.reply(RESPONSE_PRESET.stranger_approached, {
					reply_parameters: D.selectKeys(ctx.msg, ["message_id"]),
				});
			}
		}
	}
});
