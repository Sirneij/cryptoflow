<script>
	import Charts from '$lib/components/Charts.svelte';
	import { NUM_OF_COINS_TO_SHOW } from '$lib/utils/constants.js';
	import { onDestroy, onMount } from 'svelte';

	export let data,
		/** @type {import('./$types').ActionData} */
		form;

	/**
	 * @typedef {Object} Coin
	 * @property {string} id - The id of the coin.
	 * @property {string} name - The name of the coin.
	 * @property {string} symbol - The symbol of the coin.
	 * @property {string} image - The image of the coin.
	 * @property {number} market_cap_rank - The market cap rank of the coin.
	 */

	/**
	 * @type {Coin[]}
	 */
	let selectedCoins = [],
		/** @type {Number} */
		intervalId;

	$: ({ questions, coins } = data);

	const selectCoins = () => {
		const selectedCoinsSet = new Set();
		while (selectedCoinsSet.size < NUM_OF_COINS_TO_SHOW) {
			const randomIndex = Math.floor(Math.random() * coins.length);
			selectedCoinsSet.add(coins[randomIndex]);
		}
		selectedCoins = Array.from(selectedCoinsSet);
	};

	onMount(() => {
		selectCoins(); // Select coins immediately on mount
		intervalId = setInterval(selectCoins, 10000); // Select coins every 10 seconds
	});

	onDestroy(() => {
		clearInterval(intervalId); // Clear the interval when the component is destroyed
	});
</script>

<div class="flex flex-col md:flex-row text-[#efefef]">
	<!-- Left Column for Tags -->
	<div class="hidden md:block md:w-1/4 p-4 resize overflow-auto">
		<!-- Developer Profile Card -->
		<div
			class="bg-[#041014] hover:bg-black border border-black hover:border-[#145369] rounded-lg shadow p-4 mb-1"
		>
			<img
				src="https://media.licdn.com/dms/image/D4D03AQElygM4We8kqA/profile-displayphoto-shrink_800_800/0/1681662853733?e=1721865600&v=beta&t=idb1YHHzZbXHJ1MxC4Ol2ZnnbyCHq6GDtjzTzGkziLQ"
				alt="Developer"
				class="rounded-full w-24 h-24 mx-auto mb-3"
			/>
			<h3 class="text-center text-xl font-bold mb-2">John O. Idogun</h3>
			<a href="https://github.com/sirneij" class="text-center text-blue-500 block mb-2">
				@SirNeij
			</a>
			<p class="text-center">Developer & Creator of CryptoFlow</p>
		</div>
		<div
			class="bg-[#041014] p-6 rounded-lg shadow mb-6 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Coin ranks</h2>
			{#each selectedCoins as coin (coin.id)}
				<div
					class="flex items-center justify-between mb-2 border-b border-[#0a0a0a] hover:bg-[#041014] px-3 py-1"
				>
					<div class="flex items-center">
						<img
							class="w-8 h-8 rounded-full mr-2 transition-transform duration-500 ease-in-out transform hover:rotate-180"
							src={coin.image}
							alt={coin.name}
						/>
						<span class="mr-2">{coin.name}</span>
					</div>
					<span
						class="inline-block bg-blue-500 text-white text-xs px-2 rounded-full uppercase font-semibold tracking-wide"
					>
						#{coin.market_cap_rank}
					</span>
				</div>
			{/each}
		</div>
	</div>

	<div class="md:w-5/12 py-4 px-2 resize overflow-auto">
		{#if questions}
			{#each questions as question (question.id)}
				<div
					class="
				bg-[#041014] mb-1 rounded-lg shadow hover:bg-black border border-black hover:border-[#145369]"
				>
					<div class="p-4">
						<a href="/questions/{question.id}" class="text-xl font-semibold hover:text-[#2596be]">
							{question.title}
						</a>
						<!-- <p class="mt-2">{article.description}</p> -->
						<div class="mt-3 flex flex-wrap">
							{#each question.tags as tag}
								<span
									class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
								>
									{tag.name}
								</span>
							{/each}
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>

	<!-- Right Column for Charts -->
	<div class="hidden md:block md:w-1/3 px-2 py-4 resize overflow-auto">
		<div
			class="bg-[#041014] rounded-lg shadow p-4 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Charts</h2>
			<Charts {coins} {form} />
		</div>
	</div>
</div>
