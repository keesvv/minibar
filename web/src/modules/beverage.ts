export type Beverage = {
  id: string;
  description: string;
  capacity: number;
  amount: number;
  metadata: BeverageMetadata;
};

export type BeverageMetadata = {
  category?: string;
  imageUri?: string;
  alcPercent?: number;
};
