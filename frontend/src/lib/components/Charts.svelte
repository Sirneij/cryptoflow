<script>
	import { selectedTags } from '$lib/stores/tags.stores.js';
	import { filterTags } from '$lib/utils/select.custom';
	import { applyAction, enhance } from '$app/forms';
	import { notification } from '$lib/stores/notification.store';
	import ShowError from './ShowError.svelte';
	import Loader from './Loader.svelte';
	import { fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import 'chartjs-adapter-moment';

	export let coins,
		/** @type {import('../../routes/$types').ActionData} */
		form;

	/** @type {HTMLInputElement} */
	let tagInput,
		/** @type {HTMLCanvasElement} */
		lineChartContainer,
		fetching = false,
		rendered = false,
		/**
		 * @typedef {Object} CryptoData
		 * @property {Array<Number>} prices - The price data
		 * @property {Array<Number>} market_caps - The market cap data
		 * @property {Array<Number>} total_volumes - The total volume data
		 */

		/**
		 * @typedef {Object.<String, CryptoData>} CryptoDataSet
		 */

		/** @type {CryptoDataSet} */
		plotData = {},
		/** @type {CanvasRenderingContext2D | null} */

		context,
		/** @type {Chart<"line", { x: Date; y: number; }[], unknown>} */
		myChart,
		/**
		 * @typedef {Object} Dataset
		 * @property {string} label - The name of the dataset
		 * @property {Array<{x: Date, y: number}>} data - The data points in the dataset
		 * @property {boolean} fill - Whether the area under the line should be filled
		 * @property {string} borderColor - The color of the line
		 * @property {number} tension - The amount of tension to apply to the line
		 */

		/** @type {Array<Dataset>} */
		datasets = [];

	/** @type {import('../../routes/$types').SubmitFunction}*/
	const handleCoinDataFetch = async () => {
		fetching = true;
		return async ({ result }) => {
			fetching = false;
			if (result.type === 'success') {
				$notification = { message: 'Coin data fetched successfully', colorName: 'blue' };

				if (result.data) {
					plotData = result.data.marketData;
					await applyAction(result);
				}
			}
		};
	};

	onMount(() => {
		context = lineChartContainer.getContext('2d');
		if (context === null) {
			throw new Error('Failed to get 2D context for canvas');
		}
		myChart = new Chart(context, {
			type: 'line',
			data: {
				datasets: datasets
			},
			options: {
				scales: {
					x: {
						type: 'time',
						time: {
							unit: 'day'
						}
					}
				},
				responsive: true
			}
		});
		rendered = true;
	});

	/**
	 * Update the chart with new data
	 * @param {CryptoDataSet} data - The new data to update the chart with
	 */
	const updateChart = (data) => {
		for (const crypto in data) {
			Object.keys(data[crypto]).forEach((key) => {
				datasets.push({
					label: `${crypto} ${key}`,
					data: data[crypto][key].map(
						/** @param {Array<number>} item */
						(item) => {
							return {
								x: new Date(item[0]),
								y: item[1]
							};
						}
					),
					fill: false,
					borderColor: '#' + Math.floor(Math.random() * 16777215).toString(16), // Generate a random color for each dataset
					tension: 0.1
				});
			});
		}
		myChart.data.datasets = datasets;
		myChart.update();
	};

	$: if (rendered) {
		updateChart(plotData);
	}
</script>

<form action="?/getCoinData" method="POST" use:enhance={handleCoinDataFetch}>
	<ShowError {form} />
	<div style="display: flex; justify-content: space-between;">
		<div style="flex: 2; margin-right: 10px;">
			<div class="mb-6">
				<div class="mb-4">
					<label for="tag-input" class="block text-[#efefef] text-sm font-bold mb-2">
						Select Cryptocurrency
					</label>
					<input
						bind:this={tagInput}
						type="text"
						id="tag-input"
						class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be]"
						placeholder="Select up to 4 cryptocurrencies..."
						on:keyup={() => {
							filterTags(tagInput, coins);
						}}
					/>
				</div>
				<div id="suggestions" class="mt-2">
					<!-- Suggestions will go here -->
				</div>
			</div>
			<div id="selected-tags" class="mt-4">
				<!-- Selected tags will go here -->
			</div>

			<input type="hidden" name="tags" value={$selectedTags.join(',')} required />
		</div>
		<div style="flex: 1; margin-left: 10px;">
			<label for="days" class="block text-[#efefef] text-sm font-bold mb-2">Days</label>
			<input
				type="number"
				id="days"
				name="days"
				value="7"
				required
				class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be]"
				placeholder="Enter days"
			/>
		</div>
	</div>
	{#if fetching}
		<Loader width={20} message="Fetching data..." />
	{:else}
		<button
			class="px-6 py-2 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white rounded"
		>
			Fetch Coin Data
		</button>
	{/if}
</form>

<div
	in:fly={{ x: 100, duration: 1000, delay: 1000 }}
	out:fly={{ duration: 1000 }}
	class="chart-wrapper"
>
	<canvas bind:this={lineChartContainer} />
</div>
