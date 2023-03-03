import { writable } from "svelte/store";

export type config = {
    init: boolean,
    launcher: String,
    path: String
    custom: boolean
} | any

export const config = writable<config>();