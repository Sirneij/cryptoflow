<script>
	import { applyAction, enhance } from '$app/forms';
	import Loader from '$lib/components/Loader.svelte';
	import ShowError from '$lib/components/ShowError.svelte';
	import Email from '$lib/components/inputs/Email.svelte';
	import Password from '$lib/components/inputs/Password.svelte';
	import { notification } from '$lib/stores/notification.store';
	import Text from '$lib/components/inputs/Text.svelte';

	/** @type {import('./$types').ActionData} */
	export let form;

	let processing = false;

	/** @type {import('./$types').SubmitFunction} */
	const handleSignup = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			if (result.type === 'redirect') {
				$notification = { message: 'Account creation successful', colorName: 'blue' };
			}
			await applyAction(result);
		};
	};
</script>

<div class="flex justify-center items-start pt-12 min-h-screen bg-black">
	<div class="w-full md:max-w-lg">
		<form
			class="bg-[#041014] shadow-lg rounded-lg px-8 pt-6 pb-8 mb-4 text-[#efefef] hover:bg-black border border-black hover:border-[#041014]"
			method="POST"
			action="/users/signup?/register"
			use:enhance={handleSignup}
		>
			<h2 class="text-2xl font-bold mb-4 text-center">Create an account</h2>

			<ShowError {form} />
			<div class="flex mb-2 space-x-4">
				<div class="flex-1">
					<Text
						label="First name"
						id="first-name"
						name="first_name"
						placeholder="First name"
						value=""
					/>
				</div>
				<div class="flex-1">
					<Text
						label="Last name"
						id="last-name"
						name="last_name"
						placeholder="Last name"
						value=""
					/>
				</div>
			</div>
			<Email />
			<Password confirm={true} />
			<div class="flex items-center justify-between">
				{#if processing}
					<Loader width={20} message="Signing up..." />
				{:else}
					<button
						class="bg-black border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition duration-300 ease-in-out"
					>
						Sign Up
					</button>
				{/if}

				<a
					class="inline-block align-baseline font-bold text-sm hover:text-[#2596be] transition duration-300 ease-in-out"
					href="/users/login"
				>
					Have an account?
				</a>
			</div>
		</form>
	</div>
</div>
