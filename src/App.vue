<script setup lang="ts">
import { ref } from "vue";
import { choosePdfSaveDirectory, fetchMarkdownFromUrl, savePdf } from "./lib/fetchHtml";

const link = ref("https://gemini.google.com/share/9fa1c26949c2");
const readability = ref<"human" | "ai">("human");
const format = ref<"pdf" | "json">("pdf");
const pdfFileName = ref("historic-export.pdf");
const markdownOutput = ref("");
const markdownSource = ref("");
const usedFallback = ref(false);
const errorMessage = ref("");
const successMessage = ref("");
const isFetching = ref(false);
const isSavingPdf = ref(false);

function enrichMarkdown(markdown: string, sourceUrl: string): string {
  let output = markdown.trim();

  const hasUri = /https?:\/\//i.test(output);
  const hasTable = /^\s*\|.+\|\s*$/m.test(output);
  const hasImage = /!\[[^\]]*\]\([^)]+\)/.test(output);

  if (!hasUri) {
    output += `\n\nSource: ${sourceUrl}`;
  }

  if (!hasTable) {
    output += `\n\n| Field | Value |\n| --- | --- |\n| Source URL | [Open link](${sourceUrl}) |\n| Readability | ${readability.value} |\n| Output format | ${format.value} |`;
  }

  if (!hasImage) {
    const faviconUrl = `https://www.google.com/s2/favicons?sz=128&domain_url=${encodeURIComponent(sourceUrl)}`;
    output += `\n\n![Source icon](${faviconUrl})`;
  }

  return output;
}

async function onSubmit() {
  errorMessage.value = "";
  successMessage.value = "";
  markdownOutput.value = "";
  markdownSource.value = "";
  usedFallback.value = false;
  isFetching.value = true;

  try {
    const sourceUrl = link.value.trim();
    const result = await fetchMarkdownFromUrl(sourceUrl);
    markdownOutput.value = enrichMarkdown(result.markdown, sourceUrl);
    markdownSource.value = result.source;
    usedFallback.value = result.usedFallback;
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : "Failed to fetch markdown content.";
  } finally {
    isFetching.value = false;
  }
}

async function onSavePdf() {
  if (!markdownOutput.value || isSavingPdf.value) {
    return;
  }

  errorMessage.value = "";
  successMessage.value = "";
  isSavingPdf.value = true;

  try {
    const directory = await choosePdfSaveDirectory();
    if (!directory) {
      return;
    }

    const fileName = (pdfFileName.value.trim() || "historic-export.pdf").endsWith(".pdf")
      ? (pdfFileName.value.trim() || "historic-export.pdf")
      : `${pdfFileName.value.trim() || "historic-export"}.pdf`;

    const cleanDirectory = directory.replace(/[\\/]+$/, "");
    const outputPath = `${cleanDirectory}/${fileName}`;

    const savedPath = await savePdf(markdownOutput.value, outputPath);
    successMessage.value = `PDF saved: ${savedPath}`;
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : "Failed to save PDF.";
  } finally {
    isSavingPdf.value = false;
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

        <div v-if="format === 'pdf'" class="pdf-name-wrap">
          <label for="pdf-name">PDF file name</label>
          <input id="pdf-name" v-model="pdfFileName" type="text" placeholder="historic-export.pdf" />
        </div>

        <button type="submit" :disabled="isFetching">
          {{ isFetching ? "Fetching..." : "Get data" }}
        </button>
      </form>

      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      <p v-if="successMessage" class="success">{{ successMessage }}</p>

      <section v-if="markdownOutput" class="result">
        <div class="result-head">
          <h2>Markdown Output</h2>
          <div class="actions-row">
            <button
              v-if="format === 'pdf'"
              class="save-btn"
              type="button"
              :disabled="isSavingPdf"
              @click="onSavePdf"
            >
              {{ isSavingPdf ? "Saving PDF..." : "Save as PDF" }}
            </button>
            <div class="meta-row">
            <span class="chip">Source: {{ markdownSource || "unknown" }}</span>
            <span v-if="usedFallback" class="chip chip-fallback">Fallback used</span>
            </div>
          </div>
        </div>
        <pre>{{ markdownOutput }}</pre>
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

input[type="text"] {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid var(--line);
  border-radius: 12px;
  padding: 10px 12px;
  color: var(--text);
  background: #fff;
  font: 500 0.92rem/1.2 "Trebuchet MS", "Gill Sans", sans-serif;
}

input[type="url"]:focus {
  outline: 2px solid #e8aa72;
  border-color: #e8aa72;
}

input[type="text"]:focus {
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

.success {
  margin: 14px 0 0;
  color: #2f6d2d;
  font: 600 0.9rem/1.3 "Trebuchet MS", "Gill Sans", sans-serif;
}

.result {
  margin-top: 16px;
  border: 1px solid var(--line);
  border-radius: 12px;
  padding: 14px;
  background: linear-gradient(180deg, #fffdfa 0%, #fff7ee 100%);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.8);
}

.result-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  margin-bottom: 10px;
  flex-wrap: wrap;
}

.actions-row {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
}

.pdf-name-wrap {
  display: grid;
  gap: 6px;
}

.save-btn {
  margin-top: 0;
  padding: 8px 12px;
  border-radius: 9px;
  font: 700 0.78rem/1 "Trebuchet MS", "Gill Sans", sans-serif;
}

.result h2 {
  margin: 0;
  color: var(--text);
  font: 700 0.98rem/1.2 "Trebuchet MS", "Gill Sans", sans-serif;
}

.meta-row {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.chip {
  border: 1px solid #edc89b;
  border-radius: 999px;
  padding: 4px 9px;
  background: #fff;
  color: #7b522c;
  font: 600 0.72rem/1 "Trebuchet MS", "Gill Sans", sans-serif;
}

.chip-fallback {
  border-color: #e09b6c;
  color: #8f3b0e;
  background: #fff0e3;
}

.result pre {
  margin: 0;
  max-height: 340px;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-word;
  color: #4d3a27;
  font: 500 0.78rem/1.35 Consolas, "Courier New", monospace;
  border: 1px solid #f0d7b8;
  border-radius: 10px;
  padding: 10px;
  background: #fff;
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