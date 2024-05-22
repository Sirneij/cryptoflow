<script>
	import { selectedTags } from '$lib/stores/tags.stores';
	import { filterTags } from '$lib/utils/select.custom';

	export let label, id, name, placeholder, coins, value;

	/** @type {HTMLInputElement} */
	let input;
</script>

<label for={id} class="block text-[#efefef] text-sm font-bold mb-2">{label}</label>
<input
	bind:this={input}
	type="text"
	{id}
	class="w-full p-4 bg-[#0a0a0a] border border-[#145369] rounded focus:outline-none focus:border-[#2596be] text-gray-500"
	{placeholder}
	on:keyup={() => {
		filterTags(input, coins);
	}}
	disabled={$selectedTags.length >= 4}
/>

<div id="suggestions" class="mt-2">
	<!-- Suggestions will go here -->
</div>

<div id="selected-tags" class="mt-2">
	<!-- Selected tags will go here -->
</div>

<input type="hidden" {name} value={$selectedTags.join(',')} />
