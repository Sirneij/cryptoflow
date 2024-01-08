<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import ShowError from '$lib/components/ShowError.svelte';
	/** @type {import('./$types').ActionData} */
	export let form;

	/** @type {import('./$types').SubmitFunction} */
	const handleDeleteQuestion = async () => {
		return async ({ result }) => {
			await applyAction(result);
		};
	};
</script>

<div class="flex items-center justify-center min-h-96">
	<form
		class="bg-[#041014] p-6 rounded-lg shadow"
		method="post"
		action="?/delete"
		use:enhance={handleDeleteQuestion}
	>
		<ShowError {form} />

		<p class="text-red-500 p-3 text-center mb-4 italic">
			Are you sure you want to delete this question (id={$page.params.id})
		</p>

		<div class="flex justify-center mt-4">
			<a
				href="/questions/{$page.params.id}"
				class="mt-4 px-6 py-2 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white transition duration-300 ease-in-out mr-4 rounded"
			>
				Cancel
			</a>
			<button
				class="mt-4 px-6 py-2 bg-[#041014] border border-red-400 hover:border-red-700 text-red-600 rounded"
			>
				Delete Question
			</button>
		</div>
	</form>
</div>
