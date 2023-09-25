import { writable } from "svelte/store";

export type Beverage = {
  id: string;
  description: string;
  capacity: number;
  amount: number;
  metadata: BeverageMetadata;
  capabilities: BeverageCapability[];
};

export type BeverageMetadata = {
  category?: string;
  imageUri?: string;
  alcPercent?: number;
  packaging?: string;
};

export type BeverageCapability = "unit" | "shot";

export const stock = writable<Beverage[]>([]);
