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
	import { chartConfig, handleZoom } from '$lib/utils/helpers';
	import TagCoin from './inputs/TagCoin.svelte';

	export let coins,
		/** @type {import('../../routes/$types').ActionData} */
		form;

	/** @type {HTMLInputElement} */
	let tagInput,
		/** @type {HTMLCanvasElement} */
		priceChartContainer,
		/** @type {HTMLCanvasElement} */
		marketCapChartContainer,
		/** @type {HTMLCanvasElement} */
		totalVolumeChartContainer,
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
		priceChart,
		/** @type {Chart<"line", { x: Date; y: number; }[], unknown>} */
		marketCapChart,
		/** @type {Chart<"line", { x: Date; y: number; }[], unknown>} */
		totalVolumeChart,
		/** @type {CanvasRenderingContext2D|null} */
		priceContext,
		/** @type {CanvasRenderingContext2D|null} */
		marketCapContext,
		/** @type {CanvasRenderingContext2D|null} */
		totalVolumeContext;

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
		priceContext = priceChartContainer.getContext('2d');
		marketCapContext = marketCapChartContainer.getContext('2d');
		totalVolumeContext = totalVolumeChartContainer.getContext('2d');

		if (priceContext === null || marketCapContext === null || totalVolumeContext === null) {
			throw new Error('Could not get the context of the canvas element');
		}

		// Create a new configuration object for each chart
		const priceChartConfig = { ...chartConfig };
		priceChartConfig.data = { datasets: [] };
		priceChart = new Chart(priceContext, priceChartConfig);

		const marketCapChartConfig = { ...chartConfig };
		marketCapChartConfig.data = { datasets: [] };
		marketCapChart = new Chart(marketCapContext, marketCapChartConfig);

		const totalVolumeChartConfig = { ...chartConfig };
		totalVolumeChartConfig.data = { datasets: [] };
		totalVolumeChart = new Chart(totalVolumeContext, totalVolumeChartConfig);

		rendered = true;

		// Add event listeners for zooming
		priceChartContainer.addEventListener('wheel', (event) => handleZoom(event, priceChart));
		marketCapChartContainer.addEventListener('wheel', (event) => handleZoom(event, marketCapChart));
		totalVolumeChartContainer.addEventListener('wheel', (event) =>
			handleZoom(event, totalVolumeChart)
		);
	});

	/**
	 * Update the chart with new data
	 * @param {Chart<"line", { x: Date; y: number; }[], unknown>} chart - The chart to update
	 * @param {Array<Array<number>>} data - The new data to update the chart with
	 * @param {string} label - The label to use for the dataset
	 * @param {string} cryptoName - The name of the cryptocurrency
	 */
	const updateChart = (chart, data, label, cryptoName) => {
		const dataset = {
			label: `${cryptoName} ${label}`,
			data: data.map(
				/** @param {Array<number>} item */
				(item) => {
					return {
						x: new Date(item[0]),
						y: item[1]
					};
				}
			),
			fill: false,
			borderColor: '#' + Math.floor(Math.random() * 16777215).toString(16),
			tension: 0.1
		};

		chart.data.datasets.push(dataset);
		chart.update();
	};

	$: if (rendered) {
		// Clear the datasets for each chart
		priceChart.data.datasets = [];
		marketCapChart.data.datasets = [];
		totalVolumeChart.data.datasets = [];

		Object.keys(plotData).forEach(
			/** @param {string} cryptoName */
			(cryptoName) => {
				// Update each chart with the new data
				updateChart(priceChart, plotData[cryptoName].prices, 'Price', cryptoName);
				updateChart(marketCapChart, plotData[cryptoName].market_caps, 'Market Cap', cryptoName);
				updateChart(
					totalVolumeChart,
					plotData[cryptoName].total_volumes,
					'Total Volume',
					cryptoName
				);
			}
		);
	}
</script>

<form action="?/getCoinData" method="POST" use:enhance={handleCoinDataFetch}>
	<ShowError {form} />
	<div style="display: flex; justify-content: space-between;">
		<div style="flex: 2; margin-right: 10px;">
			<TagCoin
				label="Cryptocurrencies"
				id="tag-input"
				name="tags"
				value=""
				{coins}
				placeholder="Select cryptocurrencies..."
			/>
		</div>
		<div style="flex: 1; margin-left: 10px;">
			<label for="days" class="block text-[#efefef] text-sm font-bold mb-2">Days</label>
			<input
				type="number"
				id="days"
				name="days"
				value="7"
				required
				class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be] text-gray-500"
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

<div in:fly={{ x: 100, duration: 1000, delay: 1000 }} out:fly={{ duration: 1000 }}>
	<canvas bind:this={priceChartContainer} />
	<canvas bind:this={marketCapChartContainer} />
	<canvas bind:this={totalVolumeChartContainer} />
</div>
