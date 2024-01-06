<script>
	/** @type {number | null} */
	export let width;
	/** @type {string | null} */
	export let message;
</script>

<div class="loading">
	<p class="simple-loader" style={width ? `width: ${width}px` : ''} />
	{#if message}
		<p>{message}</p>
	{/if}
</div>

<style>
	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.loading p {
		margin-left: 0.5rem;
	}
	.simple-loader {
		--b: 20px; /* border thickness */
		--n: 15; /* number of dashes*/
		--g: 7deg; /* gap  between dashes*/
		--c: #2596be; /* the color */

		width: 40px; /* size */
		aspect-ratio: 1;
		border-radius: 50%;
		padding: 1px; /* get rid of bad outlines */
		background: conic-gradient(#0000, var(--c)) content-box;
		--_m: /* we use +/-1deg between colors to avoid jagged edges */ repeating-conic-gradient(
				#0000 0deg,
				#000 1deg calc(360deg / var(--n) - var(--g) - 1deg),
				#0000 calc(360deg / var(--n) - var(--g)) calc(360deg / var(--n))
			),
			radial-gradient(farthest-side, #0000 calc(98% - var(--b)), #000 calc(100% - var(--b)));
		-webkit-mask: var(--_m);
		mask: var(--_m);
		-webkit-mask-composite: destination-in;
		mask-composite: intersect;
		animation: load 1s infinite steps(var(--n));
	}
	@keyframes load {
		to {
			transform: rotate(1turn);
		}
	}
</style>
