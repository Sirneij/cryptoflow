<script>
	import { applyAction, enhance } from '$app/forms';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import { selectedTags } from '$lib/stores/tags.stores.js';
	import { filterTags } from '$lib/utils/select.custom';

	export let data;

	$: ({ coins } = data);

	/** @type {HTMLInputElement} */
	let tagInput;

	/** @type {import('./$types').ActionData} */
	export let form;

	let processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleAskQuestion = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			await applyAction(result);
		};
	};
</script>

<div class="max-w-5xl mx-auto p-4">
	<h1 class="text-3xl font-bold mb-6">Ask a Crypto Question</h1>
	<form
		class="bg-[#041014] p-6 rounded-lg shadow"
		method="post"
		action="?/ask"
		use:enhance={handleAskQuestion}
	>
		<ShowError {form} />
		<div class="mb-4">
			<label for="title" class="block text-[#efefef] text-sm font-bold mb-2">Title</label>
			<input
				type="text"
				id="title"
				name="title"
				class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be]"
				placeholder="Enter your question title"
			/>
		</div>

		<div class="mb-4">
			<label for="content" class="block text-[#efefef] text-sm font-bold mb-2">Content</label>
			<textarea
				id="content"
				name="content"
				rows="8"
				class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be]"
				placeholder="Enter your question details (markdown supported)..."
			></textarea>
		</div>

		<div>
			<div class="mb-6">
				<div class="mb-4">
					<label for="tag-input" class="block text-[#efefef] text-sm font-bold mb-2">Tags</label>
					<input
						bind:this={tagInput}
						type="text"
						id="tag-input"
						class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be]"
						placeholder="Add up to 4 tags (atleast 1 is required)..."
						on:keyup={() => {
							filterTags(tagInput, coins);
						}}
					/>
				</div>
				<div id="suggestions" class="mt-2">
					<!-- Suggestions will go here -->
				</div>
			</div>
			<div id="selected-tags" class="mt-4">
				<!-- Selected tags will go here -->
			</div>

			<input type="hidden" name="tags" value={$selectedTags.join(',')} />
		</div>

		{#if processing}
			<Loader width={20} message="Asking question..." />
		{:else}
			<button
				type="submit"
				class="bg-black border border-[#145369] hover:border-[#2596be] text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
			>
				Ask Question
			</button>
		{/if}
	</form>
</div>
