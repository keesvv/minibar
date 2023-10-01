import { derived, get, writable } from "svelte/store";
import type { Beverage } from "./beverage";
import { config } from "./config";
import { api } from "./api";

export type Order = OrderItem[];

export type OrderItem =
  | { type: "water" }
  | { type: "unit"; beverageId: Beverage["id"] }
  | { type: "shot"; beverageId: Beverage["id"] }
  | { type: "mix"; beverageIds: Beverage["id"][] };

export const order = writable<Order>([]);
export const canOrder = derived(
  [order, config],
  ([$order, $config]) => $order.length < $config?.maxOrderSize,
  true
);

export function addToOrder(item: OrderItem) {
  order.update((ord) => [...ord, item]);
}

export async function placeOrder(order: Order) {
  return api.post("orders", { json: order });
}
