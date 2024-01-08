import { writable } from 'svelte/store';

/** @type {Array<string>} */
let tags = [];

export const selectedTags = writable(tags);
