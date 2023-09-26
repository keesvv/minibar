export { default as IconShot } from "~icons/mdi/water";
export { default as IconBottle } from "~icons/mdi/bottle-soda";

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
  packaging?: BeveragePackaging;
};

export type BeveragePackaging = "bottle" | "can";

export type BeverageCapability = "unit" | "shot" | "mix";
