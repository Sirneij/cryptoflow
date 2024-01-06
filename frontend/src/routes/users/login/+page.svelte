<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import Loader from '$lib/components/Loader.svelte';
	import { receive, send } from '$lib/utils/helpers';

	/** @type {import('./$types').ActionData} */
	export let form;

	let message = '',
		processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleLogin = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
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

			{#if form?.errors}
				<!-- Error Message Display -->
				{#each form?.errors as error (error.id)}
					<p
						class="text-red-500 p-3 text-center mb-4 italic"
						in:receive={{ key: error.id }}
						out:send={{ key: error.id }}
					>
						{error.message}
					</p>
				{/each}
			{/if}
			<div class="mb-4">
				<label class="block text-sm font-bold mb-2" for="email"> Email </label>
				<input
					class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be]"
					id="email"
					name="email"
					type="email"
					placeholder="Email"
				/>
			</div>
			<div class="mb-6">
				<label class="block text-sm font-bold mb-2" for="password"> Password </label>
				<input
					class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be]"
					id="password"
					name="password"
					type="password"
					placeholder="Password"
				/>
			</div>
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
					href="/"
				>
					Forgot Password?
				</a>
			</div>
		</form>
	</div>
</div>
