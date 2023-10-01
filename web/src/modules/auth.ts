import { writable } from "svelte/store";
import { api } from "./api";

export type Auth = {
  user: string;
};

export type Credentials = {
  name: string;
};

export const session = writable<Auth>();

export function authenticate(credentials: Credentials) {
  return api.post("auth", { json: credentials, throwHttpErrors: false });
}
