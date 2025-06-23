<script setup lang="ts">
import { onBeforeMount, type Ref, ref } from "vue";
import { getAllSolves, getAvgOfN, getBestAvgOfN, getBestTime, getScramble, scrambleCube } from "../lib/invoke_wrapper";
import { Duration } from "../lib/duration";
import { Solve } from "../lib/models";
import FullCubeDisplay from "../components/FullCubeDisplay.vue";
import TimerDisplay from "../components/TimerDisplay.vue";
import SolvesList from "../components/SolvesList.vue";
import { defaultCube } from "../lib/cube";

const solves: Ref<Solve[]> = ref([]);
const scramble = ref("");
const cube = ref(defaultCube);
const ao5 = ref(new Duration());
const ao5Pb = ref(new Duration());
const pb = ref(new Duration());

async function refreshInfo() {
  solves.value = (await getAllSolves()).sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime());
  scramble.value = await getScramble();
  cube.value = await scrambleCube(scramble.value);
  ao5.value = await getAvgOfN(5);
  ao5Pb.value = await getBestAvgOfN(5);
  pb.value = await getBestTime();
}

onBeforeMount(async () => {
  await refreshInfo();
});
</script>

<template>
  <div class="flex h-full flex-row">
    <div class="flex min-w-1/5 flex-none flex-col gap-4 border-r border-zinc-800 p-3">
      <h2 class="font-bold">Stats</h2>
      <div class="grid grid-cols-2 gap-x-3">
        <span class="text-right">PB</span>
        <span>{{ pb.toString("m:ss.ff") }}</span>
        <span class="text-right">Ao5 PB</span>
        <span>{{ ao5Pb.toString("m:ss.ff") }}</span>
        <span class="text-right">Ao5</span>
        <span>{{ ao5.toString("m:ss.ff") }}</span>
      </div>
      <h2 class="font-bold">Solves ({{ solves.length }})</h2>
      <SolvesList :solves @valuechanged="refreshInfo" />
    </div>
    <div class="flex grow flex-col justify-center p-12 text-center font-mono">
      <TimerDisplay class="grow-2" @reset="refreshInfo" @stop="refreshInfo" />
      <p class="grow-3 text-2xl">{{ scramble }}</p>
      <FullCubeDisplay class="justify-self-end" :cube />
    </div>
  </div>
</template>
