<script setup>
import { ref, computed, onUnmounted } from "vue";

const totalSeconds = ref(0);
const isRunning = ref(false);
let intervalId = null;

const minutes = computed(() => Math.floor(totalSeconds.value / 60));
const seconds = computed(() => totalSeconds.value % 60);

const displayTime = computed(() => {
  const m = String(minutes.value).padStart(2, "0");
  const s = String(seconds.value).padStart(2, "0");
  return `${m}:${s}`;
});

function startPause() {
  if (isRunning.value) {
    clearInterval(intervalId);
    isRunning.value = false;
  } else {
    isRunning.value = true;
    intervalId = setInterval(() => {
      totalSeconds.value++;
    }, 1000);
  }
}

function reset() {
  clearInterval(intervalId);
  isRunning.value = false;
  totalSeconds.value = 0;
}

function setPreset(mins) {
  if (!isRunning.value) {
    totalSeconds.value = mins * 60;
  }
}

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId);
  }
});
</script>

<template>
  <div class="content">
    <div class="timer-container">
      <h1>Timer</h1>

      <div class="timer-display">
        {{ displayTime }}
      </div>

      <div class="timer-controls">
        <button @click="startPause" class="btn btn-primary">
          {{ isRunning ? "Pause" : "Start" }}
        </button>
        <button @click="reset" class="btn btn-secondary">Reset</button>
      </div>

      <div class="timer-presets">
        <h3>Quick Presets</h3>
        <div class="preset-buttons">
          <button
            @click="setPreset(5)"
            class="btn-preset"
            :disabled="isRunning"
          >
            5 min
          </button>
          <button
            @click="setPreset(10)"
            class="btn-preset"
            :disabled="isRunning"
          >
            10 min
          </button>
          <button
            @click="setPreset(25)"
            class="btn-preset"
            :disabled="isRunning"
          >
            25 min
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.content {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: clamp(24px, 4vw, 56px);
}

.timer-container {
  width: min(500px, 100%);
  text-align: center;
  padding: clamp(32px, 6vw, 48px);
  border-radius: 28px;
  background: var(--card-bg);
  border: 1px solid var(--card-border);
  box-shadow: var(--shadow);
  backdrop-filter: blur(10px);
}

h1 {
  font-family: "Fraunces", serif;
  font-size: clamp(2rem, 4vw, 2.5rem);
  margin: 0 0 32px 0;
  color: var(--ink);
}

.timer-display {
  font-size: clamp(4rem, 10vw, 6rem);
  font-weight: 600;
  font-family: "Space Grotesk", monospace;
  color: var(--accent);
  margin: 32px 0;
  padding: 24px;
  background: var(--input-bg);
  border-radius: 16px;
  border: 1px solid var(--input-border);
  box-shadow: var(--input-shadow);
}

.timer-controls {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-top: 32px;
}

.btn {
  padding: 14px 32px;
  border-radius: 12px;
  border: none;
  font-size: 1rem;
  font-weight: 600;
  font-family: "Space Grotesk", sans-serif;
  cursor: pointer;
  transition: all 200ms ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: #e07449;
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(212, 101, 60, 0.3);
}

.btn-primary:active {
  transform: translateY(0);
}

.btn-secondary {
  background: var(--card-bg);
  color: var(--ink);
  border: 1px solid var(--card-border);
}

.btn-secondary:hover {
  background: var(--input-bg);
  transform: translateY(-2px);
}

.timer-presets {
  margin-top: 40px;
  padding-top: 32px;
  border-top: 1px solid var(--card-border);
}

.timer-presets h3 {
  font-size: 1rem;
  font-weight: 600;
  color: var(--muted);
  margin: 0 0 16px 0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.preset-buttons {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.btn-preset {
  padding: 10px 20px;
  border-radius: 8px;
  border: 1px solid var(--input-border);
  background: var(--input-bg);
  color: var(--ink);
  font-size: 0.9rem;
  font-weight: 500;
  font-family: "Space Grotesk", sans-serif;
  cursor: pointer;
  transition: all 150ms ease;
}

.btn-preset:hover:not(:disabled) {
  background: var(--accent-2);
  color: white;
  border-color: var(--accent-2);
  transform: translateY(-1px);
}

.btn-preset:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
