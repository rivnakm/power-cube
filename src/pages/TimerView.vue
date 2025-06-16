<script setup lang="ts">
import { ref, onBeforeMount } from "vue";
import TimerDisplay from "../components/TimerDisplay.vue";

const scramble = ref("");

async function newScramble() {
  scramble.value = await getScramble();
}

onBeforeMount(() => {
  return newScramble();
});
</script>

<template>
  <TimerDisplay @reset="newScramble" />
  <p>{{ scramble }}</p>
</template>
