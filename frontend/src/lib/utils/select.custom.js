/**
 * Tag Selection and Suggestion Management.
 * Handles the logic for filtering tags based on user input, selecting tags, and displaying selected tags and suggestions.
 */

import { selectedTags } from '$lib/stores/tags.stores';
import { get } from 'svelte/store';

/** @type {HTMLInputElement} */
let inputFromOutside;

// Create a Tag type that has id, name, and symbol properties all of type string in jsdoc
/**
 * @typedef {Object} Tag
 * @property {string} id
 * @property {string} name
 * @property {string} symbol
 */

/**
 * Filter tags based on user input and display suggestions.
 * @file $lib/utils/select.custom.ts
 * @param {HTMLInputElement} tagInput - The input element
 * @param {Array<Tag>} allTags - All the tags
 */
export function filterTags(tagInput, allTags) {
	inputFromOutside = tagInput;
	const input = tagInput.value.toLowerCase();

	if (input.trim() === '') {
		clearSuggestions();
		return;
	}

	let $selectedTags = get(selectedTags);

	const suggestions = allTags.filter(
		(tag) =>
			(tag.id.toLowerCase().includes(input) || tag.name.toLowerCase().includes(input)) &&
			!$selectedTags.includes(tag.id)
	);

	displaySuggestions(suggestions);
}

/**
 * Select a tag and display it.
 * @file $lib/utils/select.custom.ts
 * @param {string} tagId - The tag to select
 */
function selectTag(tagId) {
	if (!get(selectedTags).includes(tagId)) {
		// Add tag to selected tags store
		selectedTags.set([...get(selectedTags), tagId]);
		displaySelectedTags();
		inputFromOutside.value = '';
		updateInputPlaceholder();
		clearSuggestions();
	} else {
		// Optional: Provide feedback to the user that the tag is already selected
		console.log('Tag already selected');
	}
}
/**
 * Clear suggestions.
 * @file $lib/utils/select.custom.ts
 */
function clearSuggestions() {
	const container = document.getElementById('suggestions');
	// @ts-ignore
	container.innerHTML = ''; // Clear suggestions
}

/**
 * Remove a tag from the selected tags.
 * @file $lib/utils/select.custom.ts
 * @param {string} tagId - The ID of the tag to remove
 */
function removeTag(tagId) {
	let $selectedTags = get(selectedTags);
	$selectedTags = $selectedTags.filter((t) => t !== tagId);
	selectedTags.set($selectedTags);
	displaySelectedTags();
	updateInputPlaceholder();
}

/**
 * Update the input placeholder text based on the number of selected tags.
 */
function updateInputPlaceholder() {
	let $selectedTags = get(selectedTags);
	if ($selectedTags.length === 4) {
		inputFromOutside.disabled = true;
		inputFromOutside.placeholder = 'Max tags reached';
	} else {
		inputFromOutside.disabled = false;
		inputFromOutside.placeholder = `Add up to ${4 - $selectedTags.length} more tags`;
	}
}

/**
 * Display suggestions to the user.
 * @file $lib/utils/select.custom.ts
 * @param {Array<Tag>} tags - The tags to display
 */
function displaySuggestions(tags) {
	/** @type {HTMLElement} */
	// @ts-ignore
	const container = document.getElementById('suggestions');
	container.innerHTML = ''; // Clear existing suggestions

	tags.forEach((tag) => {
		const div = document.createElement('div');
		div.textContent = tag.name;
		div.className = 'cursor-pointer p-2 hover:bg-[#145369]';
		div.addEventListener('click', () => selectTag(tag.id)); // Attach event listener
		container.appendChild(div);
	});
}

/**
 * Display selected tags to the user.
 * @file $lib/utils/select.custom.ts
 */
function displaySelectedTags() {
	const container = document.getElementById('selected-tags');
	// @ts-ignore
	container.innerHTML = ''; // Clear existing tags

	let $selectedTags = get(selectedTags);

	$selectedTags.forEach((tag) => {
		const span = document.createElement('span');
		span.className =
			'inline-block bg-[#145369] rounded-full px-3 py-1 text-sm font-semibold text-white mr-2 mb-2';
		span.textContent = tag;

		const removeSpan = document.createElement('span');
		removeSpan.className = 'cursor-pointer text-red-500 hover:text-red-600';
		removeSpan.textContent = ' x';
		removeSpan.onclick = () => removeTag(tag); // Attach event listener

		span.appendChild(removeSpan);
		// @ts-ignore
		container.appendChild(span);
	});
}
