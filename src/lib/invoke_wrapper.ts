import { invoke } from "@tauri-apps/api/core";
import { Cube, Solve } from "./models";
import { Duration } from "./duration";

export async function getScramble(): Promise<[string, Cube]> {
  return (await invoke("get_scramble")) as [string, Cube];
}

export async function recordSolve(solve: Solve): Promise<number> {
  return (await invoke("record_solve", {
    solve: { solveTime: solve.solveTime.totalMilliseconds, timestamp: solve.timestamp.getTime() },
  })) as number;
}

export async function getAllSolves(): Promise<Solve[]> {
  const solves = (await invoke("get_all_solves")) as { id: number; solveTime: number; timestamp: number }[];
  return solves.map(({ id, solveTime, timestamp }) => {
    return {
      id: id,
      solveTime: new Duration(0, 0, 0, solveTime),
      timestamp: new Date(timestamp),
    } as Solve;
  });
}

export async function deleteSolve(id: number): Promise<void> {
  return invoke("delete_solve", { id: id });
}

export async function getAvgOfN(n: number): Promise<Duration> {
  const avg_ms = (await invoke("get_avg_of_n", { n: n })) as number;
  return new Duration(0, 0, 0, avg_ms);
}

export async function getBestTime(): Promise<Duration> {
  const pb_ms = (await invoke("get_best_time")) as number;
  return new Duration(0, 0, 0, pb_ms);
}

export async function getBestAvgOfN(n: number): Promise<Duration> {
  const avg_ms = (await invoke("get_best_avg_of_n", { n: n })) as number;
  return new Duration(0, 0, 0, avg_ms);
}
