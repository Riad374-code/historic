import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export type MarkdownResult = {
    markdown: string;
    source: string;
    usedFallback: boolean;
};

export async function fetchMarkdownFromUrl(url: string): Promise<MarkdownResult> {
    return invoke<MarkdownResult>("fetch_markdown", { url });
}

export async function choosePdfSaveDirectory(): Promise<string | null> {
    const selected = await open({
        directory: true,
        multiple: false,
        title: "Choose folder for PDF",
    });

    if (!selected) {
        return null;
    }

    return Array.isArray(selected) ? selected[0] : selected;
}

export async function savePdf(markdown: string, outputPath: string): Promise<string> {
    return invoke<string>("create_pdf", {
        markdown,
        outputPath,
    });
}