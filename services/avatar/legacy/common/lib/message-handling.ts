import { Message } from "grammy/types";

export { type Message };

export interface TextResponseExpected {
	query: Message.TextMessage["text"];
	reply: (response: string) => Promise<Message.TextMessage>;
}
