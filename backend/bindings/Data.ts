// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BlockchainNotification } from "./BlockchainNotification";
import type { ChatMessage } from "./ChatMessage";

export type Data = { "message": ChatMessage } | { "blockchain_notification": BlockchainNotification } | { "close": Record<string, never> };