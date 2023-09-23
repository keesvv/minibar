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

export function getCategoryName(beverage: Beverage) {
  switch (beverage.metadata.category) {
    case "softDrink":
      return "Soft drink";
    case "beer":
      return "Beer";
    case "wine":
      return "Wine";
    case "spirit":
      return "Spirit";
    default:
      break;
  }
}
