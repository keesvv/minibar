import { writable } from "svelte/store";

export type Config = {
  size: SizeConfig;
  maxOrderSize: number;
};

export type SizeConfig = {
  shot: number;
};

export const config = writable<Config>();
