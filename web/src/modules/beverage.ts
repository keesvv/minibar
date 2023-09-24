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
};

export type BeverageCapabilities = {
  canShot: boolean;
};
