import { createActor, createMachine } from "xstate";
import { TelegramBot } from "../clients/mod.ts";
import { Logging } from "../lib/mod.ts";

export const Heartbeat = createActor(
	createMachine({
		initial: "awake",

		states: {
			asleep: {
				on: {
					INIT: "awake",
				},
			},

			awake: {
				on: {
					OFF: "asleep",
				},
			},
		},

		entry: ({ system }) => {
			// ...
		},
	}),
	{ systemId: "Heartbeat" },
);

Heartbeat.subscribe(({ value }) => {
	const status = `\t${value.toString().toLocaleUpperCase()}`;

	switch (value) {
		case "awake": {
			TelegramBot.catch(Logging.error);
			TelegramBot.start().catch(Logging.error);
			return Logging.yellowOutlined(`💙 ${status}`);
		}

		case "asleep": {
			TelegramBot.catch(Logging.error);
			TelegramBot.stop().catch(Logging.error);
			return Logging.redOutlined(`🖤 ${status}`);
		}
	}
});
