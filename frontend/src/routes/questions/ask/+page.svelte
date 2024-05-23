<script>
	import { applyAction, enhance } from '$app/forms';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import TagCoin from '$lib/components/inputs/TagCoin.svelte';
	import Text from '$lib/components/inputs/Text.svelte';
	import TextArea from '$lib/components/inputs/TextArea.svelte';
	import { notification } from '$lib/stores/notification.store.js';

	export let data;

	$: ({ coins } = data);

	/** @type {import('./$types').ActionData} */
	export let form;

	let processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleAskQuestion = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;

			if (result.type === 'redirect') {
				notification.set({ message: 'Question asked successfully', colorName: 'blue' });
			}
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
		<div class="mb-2">
			<Text
				label="Title"
				id="title"
				name="title"
				placeholder="Enter your question title..."
				value=""
			/>
		</div>

		<div class="mb-2">
			<TextArea
				label="Content"
				id="content"
				name="content"
				placeholder="Provide your question in detail (markdown supported) ..."
				value=""
			/>
		</div>

		<TagCoin
			label="Tags"
			id="tags"
			name="tags"
			{coins}
			placeholder="Add up to 4 tags (atleast 1 is required)..."
			value=""
		/>

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
