import { writable } from "svelte/store";
import type { Beverage } from "./beverage";

export const stock = writable<Beverage[]>([]);

export function findById(stock: Beverage[], itemId: Beverage["id"]): Beverage {
  return stock.find((s) => s.id === itemId);
}
