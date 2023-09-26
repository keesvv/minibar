import { derived, writable } from "svelte/store";
import type { Beverage } from "./beverage";
import { config } from "./config";

export type Order = OrderItem[];

export type OrderItem =
  | { type: "water" }
  | { type: "beverage"; beverageId: Beverage["id"] }
  | { type: "mix"; beverageIds: Beverage["id"][] }
  | { type: "shot"; beverageId: Beverage["id"] }
  | { type: "unit"; beverageId: Beverage["id"] };

export const order = writable<Order>([]);
export const canOrder = derived(
  [order, config],
  ([$order, $config]) => $order.length < $config?.maxOrderSize,
  true
);

export function addToOrder(item: OrderItem) {
  order.update((ord) => [...ord, item]);
}

export function placeOrder() {
  navigator.vibrate?.([50, 50]);
  order.set([]);
}
