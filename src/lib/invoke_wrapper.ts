import { invoke } from "@tauri-apps/api/core";

export async function getScramble() {
  return invoke("get_scramble");
}
