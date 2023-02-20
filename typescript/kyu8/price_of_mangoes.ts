export function mango(quantity: number, price: number): number {
  return quantity * price - Math.floor(quantity / 3) * price;
}
