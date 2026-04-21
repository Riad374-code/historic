import { invoke } from "@tauri-apps/api/core";

export async function fetchHtmlFromUrl(url: string): Promise<string> {
    return invoke<string>("fetch_html", { url });
}
