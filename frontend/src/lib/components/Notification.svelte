<script>
	import { notification } from '$lib/stores/notification.store';
	import { receive, send } from '$lib/utils/helpers';
	import { onDestroy } from 'svelte';
	/** @type {boolean} */
	let visible,
		ms = 5000,
		/** @type {any} */
		timeout,
		textColor = '--text-emerald',
		bgColor = '--bg-emerald',
		borderColor = '--border-emerald',
		text = 'Well done!:';

	/**
	 * Callback function to handle the message change.
	 * @param {string} message - The message to display.
	 * @param {number} ms - The time in milliseconds to display the message.
	 */
	const onMessageChange = (message, ms) => {
		clearTimeout(timeout);
		if (!message) {
			// hide Alert if message is empty
			visible = false;
		} else {
			// show alert
			visible = true;

			// and hide it after ms milliseconds
			if (ms > 0) timeout = setTimeout(() => (visible = false), ms);
		}
	};

	// whenever the alert store or the ms props changes run onMessageChange
	$: onMessageChange($notification.message, ms);

	// make sure we clean-up the timeout
	onDestroy(() => clearTimeout(timeout));

	$: {
		if ($notification.colorName === 'rose') {
			bgColor = '--bg-rose-100';
			borderColor = '--border-rose';
			textColor = '--text-rose';
			text = 'Oh snap!:';
		} else if ($notification.colorName === 'blue') {
			bgColor = '--bg-blue';
			borderColor = '--border-blue';
			textColor = '--text-blue';
			text = 'Heads up!:';
		} else if ($notification.colorName === 'orange') {
			bgColor = '--bg-orange';
			borderColor = '--border-orange';
			textColor = '--text-orange';
			text = 'Warning!:';
		}
	}
</script>

{#if visible}
	<button
		type="button"
		on:click={() => (visible = false)}
		class="notification"
		style:background-color={`var(${bgColor})`}
		style:color={`var(${textColor})`}
		style:border-color={`var(${borderColor})`}
		in:receive={{ key: Math.floor(Math.random() * 100) }}
		out:send={{ key: Math.floor(Math.random() * 100) }}
	>
		<strong style="margin-right: 1rem;">
			{text}
		</strong>
		{$notification.message}
	</button>
{/if}
