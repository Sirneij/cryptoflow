<script>
	import { receive, send } from '$lib/utils/helpers';

	/** @type {any}*/
	export let form;
</script>

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
