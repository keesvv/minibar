import { writable } from "svelte/store";
import type { Beverage } from "./beverage";

export type Order = OrderItem[];

export type OrderItem =
  | { type: "water" }
  | { type: "beverage"; beverageId: Beverage["id"] }
  | { type: "mix"; beverageIds: Beverage["id"][] };

export const order = writable<Order>([]);

export function addToOrder(item: OrderItem) {
  order.update((ord) => [...ord, item]);
}

export function placeOrder() {
  navigator.vibrate?.([50, 50]);
  order.set([]);
}
