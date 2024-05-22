<script>
	import { applyAction, enhance } from '$app/forms';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import TagCoin from '$lib/components/inputs/TagCoin.svelte';
	import Text from '$lib/components/inputs/Text.svelte';
	import TextArea from '$lib/components/inputs/TextArea.svelte';
	import { notification } from '$lib/stores/notification.store.js';
	import { selectedTags } from '$lib/stores/tags.stores.js';
	import { displaySelectedTags, filterTags, setInputElement } from '$lib/utils/select.custom.js';
	import { onMount } from 'svelte';

	export let data;

	$: ({ question, coins } = data);

	onMount(async () => {
		if (question) {
			// @ts-ignore
			selectedTags.set(question.tags.map((tag) => tag.id));
			displaySelectedTags();
		}
		// Set the input element in select.custom.js
		setInputElement(tagInput);
	});

	/** @type {HTMLInputElement} */
	let tagInput;

	/** @type {import('./$types').ActionData} */
	export let form;

	let processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleUpdateQuestion = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			if (result.type === 'redirect') {
				notification.set({ message: 'Question updated successfully', colorName: 'blue' });
			}
			await applyAction(result);
		};
	};
</script>

<div class="max-w-5xl mx-auto p-4">
	<form
		class="bg-[#041014] p-6 rounded-lg shadow"
		method="post"
		action="?/update"
		use:enhance={handleUpdateQuestion}
	>
		<ShowError {form} />
		<div class="mb-2">
			<Text
				label="Title"
				id="title"
				name="title"
				placeholder="Enter your question title"
				value={question.title}
			/>
		</div>

		<TextArea
			label="Content"
			id="content"
			name="content"
			placeholder="Enter your question details (markdown supported)..."
			value={question.raw_content}
		/>

		<TagCoin
			label="Tags"
			id="tag-input"
			name="tags"
			value=""
			{coins}
			placeholder={$selectedTags.length >= 4
				? 'Max tags reached'
				: `Add up to ${4 - $selectedTags.length} more tags`}
		/>

		{#if processing}
			<Loader width={20} message="Updating question..." />
		{:else}
			<button
				type="submit"
				class="bg-black border border-[#145369] hover:border-[#2596be] text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
			>
				Update Question
			</button>
		{/if}
	</form>
</div>
