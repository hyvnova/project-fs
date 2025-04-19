import ParseError from "$lib/components/ParseError.svelte";
import SearchResult from "$lib/components/SearchResult.svelte";
import type { Component } from "svelte";
import { writable } from "svelte/store";


const COL_TO_COMP_MAP: Record<string, () => Component> = {
    "SearchResult": () => SearchResult,
    "ParseError": () => ParseError
}

export const columns = writable<Record<string, Component>>({})

export function add_col(component: string) {
    console.log("Adding col", component)
    columns.update((cols) => {
        cols[component] = COL_TO_COMP_MAP[component]()
        return cols;
    })
}

export function remove_col(component: string) {
    console.log("Remove col", component)

    columns.update((cols) => {
        delete cols[component];
        return cols;
    })
}