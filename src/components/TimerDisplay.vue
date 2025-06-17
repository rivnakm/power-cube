<script setup lang="ts">
import { onMounted, onUnmounted, ref, type Ref } from "vue";
import { recordSolve } from "../lib/invoke_wrapper";
import { Timer } from "../lib/timer";
import { Solve } from "../lib/models";
import { Duration } from "../lib/duration";

const emit = defineEmits(["stop"]);

const time = ref("0:00.00");
const toStringFormat = "m:ss.ff";
const holdToStartDuration = new Duration(0, 0, 0, 500);

let timerUpdateInterval: number | undefined = undefined;
let spacebarUpdateInterval: number | undefined = undefined;
let spacebarHoldStart: Date | undefined = undefined;

const timerState: Ref<"default" | "waiting" | "ready"> = ref("default");

const timer = new Timer();

function updateTime() {
  time.value = timer.current().toString(toStringFormat);
}

function start() {
  timer.start();
  timerState.value = "default";
  timerUpdateInterval = setInterval(updateTime, 10);
}

function reset() {
  emit("stop");
  time.value = "0:00.00";
  if (timerUpdateInterval !== undefined) {
    clearInterval(timerUpdateInterval);
    timerUpdateInterval = undefined;
  }
}

async function stop() {
  if (timerUpdateInterval !== undefined) {
    clearInterval(timerUpdateInterval);
    timerUpdateInterval = undefined;

    const duration = timer.stop();

    await recordSolve({ solveTime: duration, timestamp: new Date() } as Solve);
    time.value = duration.toString(toStringFormat);
    reset();
  }
}

function cancel() {
  timer.stop();
  reset();
}

function updateSpacebarHold() {
  if (spacebarHoldStart === undefined) {
    return;
  }

  const timePressed = Duration.fromDates(spacebarHoldStart, new Date());

  if (timePressed.compareTo(holdToStartDuration) === 1) {
    timerState.value = "ready";
  }
}

async function handleKeyDownEvent(event: KeyboardEvent) {
  // don't need to handle repeating
  if (event.repeat) {
    return;
  }

  switch (event.code) {
    case "Space":
      event.preventDefault();
      event.stopPropagation();
      if (!timer.isRunning()) {
        timerState.value = "waiting";
        spacebarHoldStart = new Date();
        spacebarUpdateInterval = setInterval(updateSpacebarHold, 10);
      }
      break;
  }
}

async function handleKeyUpEvent(event: KeyboardEvent) {
  switch (event.code) {
    case "Space":
      event.preventDefault();
      event.stopPropagation();
      if (timer.isRunning()) {
        await stop();
      } else {
        clearInterval(spacebarUpdateInterval);
        spacebarUpdateInterval = undefined;
        spacebarHoldStart = undefined;
        if (timerState.value === "ready") {
          start();
        } else {
          timerState.value = "default";
        }
      }
      break;
    case "Escape":
      if (timer.isRunning()) {
        cancel();
      }
      break;
  }
}

function registerGlobalEventHandlers() {
  window.addEventListener("keydown", handleKeyDownEvent);
  window.addEventListener("keyup", handleKeyUpEvent);
}

function deregisterGlobalEventHandlers() {
  window.removeEventListener("keydown", handleKeyDownEvent);
  window.removeEventListener("keyup", handleKeyUpEvent);
}

onMounted(registerGlobalEventHandlers);
onUnmounted(deregisterGlobalEventHandlers);
</script>

<template>
  <span
    class="font-mono text-6xl"
    :class="{ 'text-amber-200': timerState === 'waiting', 'text-green-300': timerState === 'ready' }"
    >{{ time }}</span
  >
</template>
