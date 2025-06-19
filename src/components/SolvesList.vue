<script setup lang="ts">
import { deleteSolve } from "../lib/invoke_wrapper";
import { Solve } from "../lib/models";

const emit = defineEmits(["valuechanged"]);

defineProps<{
  solves: Solve[];
}>();

async function execDeleteSolve(id: number) {
  await deleteSolve(id);
  emit("valuechanged");
}
</script>

<template>
  <div class="shrink overflow-auto">
    <div v-for="solve in solves" :key="solve.id" class="flex flex-row">
      <div class="flex grow flex-col">
        <div class="p-1 px-2 align-middle text-lg font-bold">{{ solve.solveTime.toString("m:ss.ff") }}</div>
        <div class="p-1 px-2 align-middle text-sm font-bold text-zinc-600">{{ solve.timestamp.toLocaleString() }}</div>
      </div>
      <div class="align-middle">
        <button class="p-2 text-zinc-600 hover:text-red-400" @click="execDeleteSolve(solve.id)">
          <i class="fa-solid fa-xmark"></i>
        </button>
      </div>
    </div>
  </div>
</template>
