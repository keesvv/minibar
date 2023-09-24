import { writable } from "svelte/store";

export type Beverage = {
  id: string;
  description: string;
  capacity: number;
  amount: number;
  metadata: BeverageMetadata;
  capabilities: BeverageCapabilities;
};

export type BeverageMetadata = {
  category?: string;
  imageUri?: string;
  alcPercent?: number;
  packaging?: string;
};

export type BeverageCapabilities = {
  unit: boolean;
  shot: boolean;
};

export const stock = writable<Beverage[]>([]);
