<script setup>
import { ref } from "vue";

const notes = ref("");
</script>

<template>
  <div class="content">
    <div class="braindump-container">
      <h1>Brain Dump</h1>
      <textarea
        v-model="notes"
        placeholder="Let your thoughts flow freely..."
        class="braindump-textarea"
      ></textarea>
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

.braindump-container {
  width: min(700px, 100%);
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

.braindump-textarea {
  width: 100%;
  min-height: 400px;
  padding: 20px;
  border-radius: 16px;
  border: 1px solid var(--input-border);
  background: var(--input-bg);
  color: var(--ink);
  font-family: "Space Grotesk", sans-serif;
  font-size: 1rem;
  line-height: 1.6;
  resize: vertical;
  box-shadow: var(--input-shadow);
  transition: all 200ms ease;
}

.braindump-textarea::placeholder {
  color: var(--placeholder);
}

.braindump-textarea:focus {
  outline: none;
  border-color: var(--accent-2);
  box-shadow: var(--ring);
}
</style>
