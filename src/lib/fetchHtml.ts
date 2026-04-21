import { invoke } from "@tauri-apps/api/core";

export async function fetchHtmlFromUrl(url: string): Promise<string> {
    return invoke<string>("fetcher", { url });
}

export async function html2markdown(body: string): Promise<string> {
    return invoke<string>("html_to_markdown", { body });
}

export async function fetchMarkdownFromUrl(url: string): Promise<string> {
    return invoke<string>("fetch_markdown", { url });
}