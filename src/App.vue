<script setup lang="ts">
import { ref } from "vue";
import { fetchHtmlFromUrl } from "./lib/fetchHtml";

const link = ref("https://gemini.google.com/share/9fa1c26949c2");
const readability = ref<"human" | "ai">("human");
const format = ref<"pdf" | "json">("pdf");
const fetchedHtml = ref("");
const errorMessage = ref("");
const isFetching = ref(false);

async function onSubmit() {
  errorMessage.value = "";
  fetchedHtml.value = "";
  isFetching.value = true;

  try {
    fetchedHtml.value = await fetchHtmlFromUrl(link.value.trim());
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : "Failed to fetch HTML.";
  } finally {
    isFetching.value = false;
  }
}
</script>

<template>
  <div class="page">
    <main class="card">
      <p class="eyebrow">Historic</p>
      <h1>Extract clean data from a link</h1>

      <form class="extract-form" @submit.prevent="onSubmit">
        <label for="link">Paste Link</label>
        <input
          id="link"
          v-model="link"
          type="url"
          placeholder="https://example.com/article"
          required
        />

        <fieldset>
          <legend>Readability</legend>
          <div class="option-row">
            <label class="option">
              <input v-model="readability" type="radio" value="human" name="readability" />
              <span>Human Readable</span>
            </label>
            <label class="option">
              <input v-model="readability" type="radio" value="ai" name="readability" />
              <span>AI Readable</span>
            </label>
          </div>
        </fieldset>

        <fieldset>
          <legend>Format</legend>
          <div class="option-row">
            <label class="option">
              <input v-model="format" type="radio" value="pdf" name="format" />
              <span>PDF</span>
            </label>
            <label class="option">
              <input v-model="format" type="radio" value="json" name="format" />
              <span>JSON</span>
            </label>
          </div>
        </fieldset>

        <button type="submit" :disabled="isFetching">
          {{ isFetching ? "Fetching..." : "Get data" }}
        </button>
      </form>

      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>

      <section v-if="fetchedHtml" class="result">
        <h2>Fetched HTML</h2>
        <pre>{{ fetchedHtml }}</pre>
      </section>
    </main>
  </div>
</template>

<style scoped>
.page {
  --bg: #fff5e8;
  --surface: #fffcf7;
  --line: #f0d2ad;
  --accent: #d97728;
  --accent-strong: #b95f1a;
  --text: #2d2014;
  --muted: #7e6245;

  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 28px;
  background:
    radial-gradient(circle at 15% 10%, #ffd4a6 0%, transparent 36%),
    radial-gradient(circle at 85% 92%, #ffc287 0%, transparent 32%),
    var(--bg);
}

.card {
  width: min(640px, 100%);
  padding: 28px;
  border: 1px solid var(--line);
  border-radius: 18px;
  background: var(--surface);
  box-shadow: 0 14px 28px rgba(102, 66, 31, 0.12);
  animation: rise 0.45s ease-out;
}

.eyebrow {
  margin: 0;
  color: var(--accent);
  font: 700 0.82rem/1 "Trebuchet MS", "Gill Sans", sans-serif;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

h1 {
  margin: 10px 0 22px;
  color: var(--text);
  font: 700 clamp(1.55rem, 2.4vw, 2rem) / 1.2 "Palatino Linotype", "Book Antiqua", Palatino, serif;
}

.extract-form {
  display: grid;
  gap: 14px;
}

label,
legend {
  color: var(--text);
  font: 600 0.95rem/1.2 "Trebuchet MS", "Gill Sans", sans-serif;
}

input[type="url"] {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid var(--line);
  border-radius: 12px;
  padding: 12px 14px;
  color: var(--text);
  background: #fff;
  font: 500 0.95rem/1.2 "Trebuchet MS", "Gill Sans", sans-serif;
}

input[type="url"]:focus {
  outline: 2px solid #e8aa72;
  border-color: #e8aa72;
}

fieldset {
  margin: 2px 0 0;
  padding: 12px;
  border: 1px solid var(--line);
  border-radius: 12px;
}

.option-row {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.option {
  display: flex;
  align-items: center;
  gap: 8px;
  border: 1px solid var(--line);
  border-radius: 10px;
  padding: 10px;
  background: #fff;
  color: var(--muted);
}

.option:has(input:checked) {
  border-color: #e8aa72;
  background: #fff5e8;
  color: var(--text);
}

button {
  margin-top: 8px;
  border: 0;
  border-radius: 12px;
  padding: 12px 16px;
  color: #fff;
  background: linear-gradient(90deg, var(--accent), var(--accent-strong));
  font: 700 0.96rem/1 "Trebuchet MS", "Gill Sans", sans-serif;
  cursor: pointer;
  transition: transform 0.2s ease, filter 0.2s ease;
}

button:hover {
  transform: translateY(-1px);
  filter: brightness(1.03);
}

button:active {
  transform: translateY(0);
}

button:disabled {
  cursor: not-allowed;
  filter: grayscale(0.2);
  opacity: 0.8;
}

.error {
  margin: 14px 0 0;
  color: #a13b11;
  font: 600 0.9rem/1.3 "Trebuchet MS", "Gill Sans", sans-serif;
}

.result {
  margin-top: 16px;
  border: 1px solid var(--line);
  border-radius: 12px;
  padding: 12px;
  background: #fff;
}

.result h2 {
  margin: 0 0 8px;
  color: var(--text);
  font: 700 0.98rem/1.2 "Trebuchet MS", "Gill Sans", sans-serif;
}

.result pre {
  margin: 0;
  max-height: 220px;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-word;
  color: #4d3a27;
  font: 500 0.78rem/1.35 Consolas, "Courier New", monospace;
}

@keyframes rise {
  from {
    opacity: 0;
    transform: translateY(8px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 640px) {
  .page {
    padding: 16px;
  }

  .card {
    padding: 20px;
  }

  .option-row {
    grid-template-columns: 1fr;
  }
}
</style>