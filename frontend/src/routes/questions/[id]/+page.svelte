<script>
	import { page } from '$app/stores';
	import Logo from '$lib/assets/logo.png';
	import { formatCoinName, formatPrice, getCoinsPricesServer } from '$lib/utils/helpers.js';
	import { onMount } from 'svelte';

	export let data;

	$: ({ question, answers } = data);

	/** @type {Array<{"name": String, "price": number}>} */
	let coinPrices = [];

	onMount(async () => {
		if (question) {
			// @ts-ignore
			const tagsString = question.tags.map((tag) => tag.id).join(',');
			coinPrices = await getCoinsPricesServer($page.data.fetch, tagsString, 'usd');
		}
	});
</script>

<div class="max-w-5xl mx-auto p-4">
	<!-- Stats Section -->
	<div class="bg-[#0a0a0a] p-6 rounded-lg shadow mb-6 flex justify-between items-center">
		<p>Created: {question.created_at}</p>
		<p>Last Updated: {question.updated_at}</p>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-12 gap-4">
		<!-- Main Content -->
		<div class="md:col-span-9">
			<!-- Question Section -->
			<div class="bg-[#041014] p-6 rounded-lg shadow mb-6 border border-black">
				<h1 class="text-2xl font-bold mb-4">{question.title}</h1>
				<p>{@html question.content}</p>
				<div class="flex mt-4">
					{#each question.tags as tag}
						<span
							class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
						>
							{tag.name.toLowerCase()}
						</span>
					{/each}
				</div>
				<div class="flex justify-end mt-4">
					<button class="mr-2 text-blue-500 hover:text-blue-600">Edit</button>
					<button class="mr-2 text-red-500 hover:text-red-600">Delete</button>
					<button class="text-yellow-500 hover:text-yellow-600">Flag</button>
				</div>
				<hr class="my-4" />
				<div class="flex justify-end items-center">
					<span class="mr-3">
						{question.author.first_name + ' ' + question.author.last_name}
					</span>
					<img
						src={question.author.thumbnail ? question.author.thumbnail : Logo}
						alt={question.author.first_name + ' ' + question.author.last_name}
						class="h-10 w-10 rounded-full"
					/>
				</div>
			</div>

			<!-- Answers Section -->
			<h2 class="text-xl font-bold mb-4">Answers</h2>

			<!-- Answers Section -->
			{#each answers as answer (answer.id)}
				<div class="bg-[#041014] p-6 rounded-lg shadow mb-4">
					<p>{@html answer.content}</p>
					<hr class="my-4" />
					<div class="flex justify-end items-center">
						<span class="mr-3">{answer.author.first_name + ' ' + answer.author.last_name}</span>
						<img
							src={answer.author.thumbnail ? question.author.thumbnail : Logo}
							alt={answer.author.first_name + ' ' + answer.author.last_name}
							class="h-10 w-10 rounded-full"
						/>
					</div>
				</div>
				<div class="flex justify-end mt-4">
					<button class="mr-2 text-blue-500 hover:text-blue-600">Edit</button>
					<button class="mr-2 text-red-500 hover:text-red-600">Delete</button>
					<button class="text-yellow-500 hover:text-yellow-600">Flag</button>
				</div>
			{:else}
				<div class="bg-[#041014] p-6 rounded-lg shadow mb-4">
					<p>No answers yet.</p>
				</div>
			{/each}

			<!-- Post Answer Section -->
			<form class="bg-[#041014] p-6 rounded-lg shadow">
				<h2 class="text-xl font-bold mb-4">Your Answer</h2>
				<textarea
					class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:border-[#2596be] focus:outline-none"
					rows="6"
					placeholder="Write your answer here (markdown supported)..."
				></textarea>
				<button
					class="mt-4 px-6 py-2 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-white rounded"
				>
					Post Your Answer
				</button>
			</form>
		</div>

		<!-- Right Sidebar -->
		<div class="md:col-span-3">
			<h2 class="text-xl font-semibold mb-4">Current prices</h2>
			<div
				class="bg-[#041014] rounded-lg shadow p-4 hover:bg-black border border-black hover:border-[#145369]"
			>
				<div class="space-y-4">
					{#each coinPrices as coin (coin.name)}
						<div class="bg-[#145369] p-3 rounded-lg text-center">
							<p class="text-3xl font-bold">
								<span class="text-base">$</span>{formatPrice(coin.price)}
							</p>
							<h3 class="text-lg">{formatCoinName(coin.name)}</h3>
						</div>
					{/each}
				</div>
			</div>
		</div>
	</div>
</div>
