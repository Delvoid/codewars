export class G964 {
  public static nbDig(n: number, d: number): number {
    let count = 0;
    for (let i = 0; i <= n; i++) {
      const square = i * i;
      const squareString = square.toString();
      for (let j = 0; j < squareString.length; j++) {
        if (squareString[j] == d.toString()) {
          count++;
        }
      }
    }
    return count;
  }
}

export class G964ANOTHER {
  public static nbDig(n: number, d: number): number {
    let count: number = 0;
    for (let k: number = 0; k <= n; k++) {
      count += (k * k).toString().split(d.toString()).length - 1;
    }
    return count;
  }
}
