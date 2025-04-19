import { writable } from "svelte/store";

export const parse_error = writable<string | null>(null);
