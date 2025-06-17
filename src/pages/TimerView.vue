<script setup lang="ts">
import { onBeforeMount, type Ref, ref } from "vue";
import { getAllSolves, getAvgOfN, getBestAvgOfN, getBestTime, getScramble } from "../lib/invoke_wrapper";
import { Duration } from "../lib/duration";
import { Solve } from "../lib/models";
import TimerDisplay from "../components/TimerDisplay.vue";
import SolvesList from "../components/SolvesList.vue";

const solves: Ref<Solve[]> = ref([]);
const scramble = ref("");
const ao5 = ref(new Duration());
const ao5Pb = ref(new Duration());
const pb = ref(new Duration());

async function refreshInfo() {
  solves.value = (await getAllSolves()).sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime());
  scramble.value = await getScramble();
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
    <div class=" flex flex-col gap-4 h-full min-w-1/6 flex-none border-r border-zinc-800 p-3">
      <div>
        <h2 class="font-bold">Stats</h2>
        <div class="grid grid-cols-2">
          <span>PB</span>
          <span>{{ pb.toString("m:ss.ff") }}</span>
          <span>Ao5 PB</span>
          <span>{{ ao5Pb.toString("m:ss.ff") }}</span>
          <span>Ao5</span>
          <span>{{ ao5.toString("m:ss.ff") }}</span>
        </div>
      </div>
      <div>
        <h2 class="font-bold">Solves ({{ solves.length }})</h2>

        <SolvesList :solves @valuechanged="refreshInfo" />
      </div>
    </div>
    <div class="grow p-12 text-center font-mono">
      <TimerDisplay @reset="refreshInfo" @stop="refreshInfo" />
      <p class="text-2xl">{{ scramble }}</p>
    </div>
  </div>
</template>
