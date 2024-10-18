import GroqClient, { type Groq } from "groq-sdk";
import { GROQ_API_TOKEN } from "../config.ts";

export const instance = new GroqClient({ apiKey: GROQ_API_TOKEN });

export interface GroqConversation {
	temperature?: number;
	messages: Groq.Chat.CompletionCreateParams.Message[];
}

export const handle = async ({ temperature = 1, messages }: GroqConversation) =>
	await instance.chat.completions
		.create({ model: "mixtral-8x7b-32768", messages, temperature })
		.then((chatCompletion): string =>
			chatCompletion.choices.at(0)?.message?.content ?? "[ No response ]"
		);
