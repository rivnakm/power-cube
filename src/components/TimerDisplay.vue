<script setup lang="ts">
import { ref } from "vue";

const time = ref("0:00.00");
const tostring_format = "%M:%S.%L";

let start_time = null;
let end_time = null;
let call_interval = undefined;

function update_time() {
  const duration = new Duration(start_time, new Date());
  time.value = duration.toString(tostring_format);
}

function start_stop() {
  // Stop
  if (call_interval !== undefined) {
    clearInterval(call_interval);
    call_interval = undefined;
    end_time = new Date();
    time.value = new Duration(start_time, end_time).toString(tostring_format);
    return;
  }

  // Start
  start_time = new Date();
  call_interval = setInterval(update_time, 10);
}
</script>

<template>
  {{ time }}
  <button @click="start_stop">start_stop</button>
</template>
