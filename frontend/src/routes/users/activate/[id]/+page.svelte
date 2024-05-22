<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import { notification } from '$lib/stores/notification.store';

	/** @type {import('./$types').ActionData} */
	export let form;

	let processing = false,
		message = '',
		token = '';

	/** @type {import('./$types').SubmitFunction} */
	const handleActivate = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			if (result.type === 'redirect') {
				$notification = { message: 'Account activated successfully', colorName: 'blue' };
			}
			await applyAction(result);
		};
	};

	if ($page.url.searchParams.get('message')) {
		message = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}

	$: {
		if (token.replace(/[-\s]/g, '').length % 3 === 0 && token.replace(/[-\s]/g, '').length > 0) {
			token = token
				.replace(/[-\s]/g, '')
				.replace(/(.{3})(.{3})?/g, '$1 - $2 ')
				.trim();
		}
	}
</script>

<div class="flex justify-center items-start pt-12 min-h-screen bg-black">
	<div class="w-full md:max-w-md">
		<form
			class="bg-[#041014] shadow-lg rounded-lg px-8 pt-6 pb-8 mb-4 text-[#efefef] hover:bg-black border border-black hover:border-[#041014]"
			method="POST"
			use:enhance={handleActivate}
		>
			<h2 class="text-2xl font-bold mb-4 text-center">Activate account</h2>

			{#if message}
				<p class="text-center text-[#efefef] mb-4">{message}</p>
			{/if}

			<ShowError {form} />
			<div class="mb-4">
				<label class="block text-sm font-bold mb-2" for="token">One Time Token</label>
				<input
					bind:value={token}
					class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be] text-gray-500"
					id="token"
					name="token"
					type="text"
					placeholder="Token"
					maxlength="9"
				/>
			</div>

			<div class="flex items-center justify-between">
				{#if processing}
					<Loader width={20} message="Activating..." />
				{:else}
					<button
						class="bg-black border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-300 ease-in-out"
					>
						Activate
					</button>
				{/if}
			</div>
		</form>
	</div>
</div>
