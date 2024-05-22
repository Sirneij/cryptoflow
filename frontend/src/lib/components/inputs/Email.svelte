<script>
	import { isValidEmail } from '$lib/utils/helpers';

	let email = '',
		isValid = false;
	$: {
		if (email.length > 0) {
			isValid = isValidEmail(email);
		}
	}
</script>

<div class="mb-2 relative">
	<label class="block text-sm font-bold mb-2" for="email"> Email </label>
	<input
		bind:value={email}
		class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be] text-gray-500"
		id="email"
		name="email"
		type="email"
		placeholder="Email"
		required
	/>
	{#if email.length > 0}
		{#if !isValid}
			<small class="text-red-500 text-xs italic">Invalid email address</small>
		{/if}
		<span
			title="Email is {isValid ? 'valid' : 'invalid'}"
			class="absolute inset-y-0 right-0 pr-3 flex items-center"
			class:pt-4={isValid}
			class:pb-2={!isValid}
		>
			{#if isValid}
				<svg
					class="h-6 w-6 text-green-500"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M5 13l4 4L19 7"
					/>
				</svg>
			{:else}
				<svg
					class="h-6 w-6 text-red-500"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			{/if}
		</span>
	{/if}
</div>
