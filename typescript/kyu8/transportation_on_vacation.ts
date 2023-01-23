// Every day you rent the car costs $40. If you rent the car for 7 or more days, you get $50 off your total.
// Alternatively, if you rent the car for 3 or more days, you get $20 off your total.
export function rentalCarCost(d: number): number {
  const PER_DAY_COST = 40;
  const cost = PER_DAY_COST * d;
  if (d >= 7) return cost - 50;
  if (d >= 3) return cost - 20;
  // 3 or less
  return cost;
}
