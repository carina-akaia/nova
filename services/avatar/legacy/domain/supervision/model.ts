import { createStore } from "effector";
import { PERSONALITY_PRESET } from "../../common/config.ts";

export type Message = {
	metadata: {
		channelId: string;

		sender: {
			role: "system" | "agent" | "user";
		};
	};

	payload: {
		content: string;
	};
};

export const $systemInstructions = createStore<string>(JSON.stringify(PERSONALITY_PRESET), {
	name: "System instructions",
});
