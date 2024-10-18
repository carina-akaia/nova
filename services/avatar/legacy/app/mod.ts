import { Heartbeat } from "../common/services/mod.ts";
import "../domain/entities/mod.ts";

export const init = () => {
	Heartbeat.start();
};
