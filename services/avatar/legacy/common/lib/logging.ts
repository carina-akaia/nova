import * as Colors from "std/fmt/colors.ts";

const separator = `\n-----------------------------------------------\n`;

export const regular = console.log;

const outlined = (message: string) =>
	`${separator} ${(new Date()).toLocaleString()}: ${message} ${separator}`;

const timeStart = (message: string) => `${(new Date()).toLocaleString()}: ${message}`;

const underlined = (message: string) => ` ${message}${separator}`;

export const blue = (message: string) => regular(Colors.blue(message));

export const green = (message: string) => regular(Colors.green(message));

export const red = (message: string) => regular(Colors.red(message));

export const redOutlined = (message: string) => red(outlined(message));

export const yellow = (message: string) => regular(Colors.yellow(message));

export const yellowOutlined = (message: string) => yellow(outlined(message));

export const info = (message: string) => blue(underlined(timeStart(message)));

export const success = (message: string) => green(underlined(timeStart(message)));

export const warn = (message: string) => yellow(underlined(timeStart(message)));

export const error = ({ message }: Error) => red(underlined(timeStart(message)));
