import { writable } from "svelte/store";

export const parse_errors = writable<string[]>([])