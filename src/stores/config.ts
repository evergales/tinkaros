import { writable } from "svelte/store";

export type config = {
    init: boolean,
    launcher: String,
    path: String
    custom: boolean,
    check_tauri_update: boolean,
    max_concurrent_downloads: number,
    bleeding_edge_updates: boolean
} | any

export const config = writable<config>();