<script>
	import { PASSWORD_ERROR_MESSAGE } from '$lib/utils/constants';
	import { isValidPasswordMedium, isValidPasswordStrong } from '$lib/utils/helpers';

	export let confirm;

	let password = '',
		passwordStrength = '',
		confirmPassword = '',
		match = false;

	$: {
		if (confirm && password.length >= 8 && /[A-Za-z]/.test(password) && /[0-9]/.test(password)) {
			if (isValidPasswordStrong(password)) {
				passwordStrength = 'strong';
			} else if (isValidPasswordMedium(password)) {
				passwordStrength = 'medium';
			} else {
				passwordStrength = 'weak';
			}
		} else {
			passwordStrength = 'invalid';
		}
	}

	$: {
		if (confirm && confirmPassword.length > 0) {
			match = password === confirmPassword;
		}
	}
</script>

<div class="mb-2" class:relative={confirm}>
	<label class="block text-sm font-bold mb-2" for="password"> Password </label>
	<input
		bind:value={password}
		class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be] text-gray-500"
		id="password"
		name="password"
		type="password"
		placeholder="Password"
		required
	/>
	{#if password.length > 0 && confirm}
		{#if passwordStrength === 'invalid'}
			<small class="text-red-500 text-xs italic">{PASSWORD_ERROR_MESSAGE}</small>
		{/if}
		<span
			title="Password strength is {passwordStrength === 'strong'
				? 'strong'
				: passwordStrength === 'medium'
					? 'medium'
					: passwordStrength === 'weak'
						? 'weak'
						: 'invalid'}"
			class="absolute inset-y-0 right-0 pr-3 flex items-center"
			class:pt-4={passwordStrength !== 'invalid'}
			class:pb-8={passwordStrength === 'invalid'}
		>
			{#if passwordStrength === 'strong'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6 text-green-500"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
					></path>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 12a9 9 0 01-9 9v-9m9 0a9 9 0 00-9-9v9z"
					></path>
				</svg>
			{:else if passwordStrength === 'medium'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6 text-blue-500"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"
					></path>
				</svg>
			{:else if passwordStrength === 'weak'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6 text-yellow-500"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01"
					></path>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15h.01"
					></path>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19h.01"
					></path>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 23h.01"
					></path>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 3a9 9 0 110 18 9 9 0 010-18z"
					></path>
				</svg>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					class="h-6 w-6 text-red-500"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					></path>
				</svg>
			{/if}
		</span>
	{/if}
</div>

{#if confirm}
	<div class="mb-2 relative">
		<label class="block text-sm font-bold mb-2" for="confirm-password"> Confirm Password </label>
		<input
			bind:value={confirmPassword}
			class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be] text-gray-500"
			id="confirm-password"
			name="confirm_password"
			type="password"
			placeholder="Confirm Password"
			required
		/>
		{#if confirmPassword.length > 0}
			{#if !match}
				<small class="text-red-500 text-xs italic">Passwords do not match</small>
			{/if}
			<span
				title="Passwords {match ? 'match' : 'do not match'}"
				class="absolute inset-y-0 right-0 pr-3 flex items-center"
				class:pt-4={match}
				class:pb-2={!match}
			>
				{#if match}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						class="h-6 w-6 text-green-500"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"
						></path>
					</svg>
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						class="h-6 w-6 text-red-500"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						></path>
					</svg>
				{/if}
			</span>
		{/if}
	</div>
{/if}
