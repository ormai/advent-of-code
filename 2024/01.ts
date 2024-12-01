import { readFileSync } from "fs";

function parseColumns(): [number[], number[]] {
  const input = readFileSync("./input", "utf-8");
  const left: number[] = [], right: number[] = [];
  for (const line of input.split("\n").slice(0, -1)) {
    const [n1, n2] = line.split("   ").map(Number);
    left.push(n1);
    right.push(n2);
  }
  return [left, right];
}

function distance(left: Array<number>, right: Array<number>): number {
  left.sort()
  right.sort()
  return left.reduce((acc, v, i) => acc + Math.abs(v - right[i]), 0)
}

function similarityScore(left: number[], right: number[]) {
  return left.reduce((acc, v) => acc + right.filter(n => n == v).length * v, 0)
}

const [left, right] = parseColumns();
console.log(distance(left, right))
console.log(similarityScore(left, right))
