<script setup lang="ts">
import { ref } from "vue";
import { Timer } from "../lib/timer";

const time = ref("0:00.00");
const tostringFormat = "m:ss.ff";

let callInterval = undefined;

const timer = new Timer();

function updateTime() {
  time.value = timer.current().toString(tostringFormat);
}

function start() {
  timer.start();
  callInterval = setInterval(updateTime, 10);
}

function stop() {
  if (callInterval !== undefined) {
    clearInterval(callInterval);
    callInterval = undefined;

    time.value = timer.stop().toString(tostringFormat);
    return;
  }
}

function reset() {
  time.value = "0:00.00";
}
</script>

<template>
  {{ time }}
  <button @click="start">start</button>
  <button @click="stop">stop</button>
  <button @click="reset">reset</button>
</template>
