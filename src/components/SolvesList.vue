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
  <table>
    <tr v-for="solve in solves" :key="solve.id" class="h-10">
      <td class="p-1 px-2 align-middle text-lg font-bold">{{ solve.solveTime.toString("m:ss.ff") }}</td>
      <td class="p-1 px-2 align-middle text-sm font-bold text-zinc-600">{{ solve.timestamp.toLocaleDateString() }}</td>
      <td class="p-1 align-middle">
        <button @click="execDeleteSolve(solve.id)">
          <i class="fa-solid fa-xmark text-zinc-600 hover:text-red-400"></i>
        </button>
      </td>
    </tr>
  </table>
</template>
