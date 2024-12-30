/// <reference types="svelte" />
/// <reference types="vite/client" />

import { Window as KeplrWindow } from "@keplr-wallet/types";
import type { I18n } from "i18n-js";

declare global {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface Window extends KeplrWindow {
    ini:I18n
  }
}
