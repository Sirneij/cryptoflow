<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import Email from '$lib/components/inputs/Email.svelte';
	import Password from '$lib/components/inputs/Password.svelte';
	import { notification } from '$lib/stores/notification.store';

	/** @type {import('./$types').ActionData} */
	export let form;

	let message = '',
		processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleLogin = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			if (result.type === 'redirect') {
				$notification = { message: 'Login successful', colorName: 'blue' };
			}
			await applyAction(result);
		};
	};

	if ($page.url.searchParams.get('message')) {
		message = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}
</script>

<div class="flex justify-center items-start pt-12 min-h-screen bg-black">
	<div class="w-full md:max-w-md">
		<form
			class="bg-[#041014] shadow-lg rounded-lg px-8 pt-6 pb-8 mb-4 text-[#efefef] hover:bg-black border border-black hover:border-[#041014]"
			method="POST"
			action="/users/login?/login"
			use:enhance={handleLogin}
		>
			<h2 class="text-2xl font-bold mb-4 text-center">Login</h2>

			{#if message}
				<p class="text-center text-[#efefef] mb-4">{message}</p>
			{/if}

			<ShowError {form} />
			<Email />
			<Password confirm={false} />
			<div class="flex items-center justify-between">
				{#if processing}
					<Loader width={20} message="Logging in..." />
				{:else}
					<button
						class="bg-black border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-300 ease-in-out"
					>
						Sign In
					</button>
				{/if}

				<a
					class="inline-block align-baseline font-bold text-sm hover:text-[#2596be] transition duration-300 ease-in-out"
					href="/users/signup"
				>
					No account? Register
				</a>
			</div>
		</form>
	</div>
</div>
