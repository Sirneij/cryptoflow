// @ts-nocheck
import { quintOut } from 'svelte/easing';
import { crossfade } from 'svelte/transition';

export const [send, receive] = crossfade({
	duration: (d) => Math.sqrt(d * 200),

	// eslint-disable-next-line no-unused-vars
	fallback(node, params) {
		const style = getComputedStyle(node);
		const transform = style.transform === 'none' ? '' : style.transform;

		return {
			duration: 600,
			easing: quintOut,
			css: (t) => `
                transform: ${transform} scale(${t});
                opacity: ${t}
            `
		};
	}
});

/**
 * Validates an email field
 * @file lib/utils/helpers/input.validation.ts
 * @param {string} email - The email to validate
 */
export const isValidEmail = (email) => {
	const EMAIL_REGEX =
		/[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?/;
	return EMAIL_REGEX.test(email.trim());
};
/**
 * Validates a strong password field
 * @file lib/utils/helpers/input.validation.ts
 * @param {string} password - The password to validate
 */
export const isValidPasswordStrong = (password) => {
	const strongRegex = new RegExp('^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*])(?=.{8,})');

	return strongRegex.test(password.trim());
};
/**
 * Validates a medium password field
 * @file lib/utils/helpers/input.validation.ts
 * @param {string} password - The password to validate
 */
export const isValidPasswordMedium = (password) => {
	const mediumRegex = new RegExp(
		'^(((?=.*[a-z])(?=.*[A-Z]))|((?=.*[a-z])(?=.*[0-9]))|((?=.*[A-Z])(?=.*[0-9])))(?=.{6,})'
	);

	return mediumRegex.test(password.trim());
};

/**
 * Test whether or not an object is empty.
 * @param {Record<string, string>} obj - The object to test
 * @returns `true` or `false`
 */

export function isEmpty(obj) {
	for (const _i in obj) {
		return false;
	}
	return true;
}

/**
 * Handle all GET requests.
 * @file lib/utils/helpers.js
 * @param {typeof fetch} sveltekitFetch - Fetch object from sveltekit
 * @param {string} targetUrl - The URL whose resource will be fetched.
 * @param {RequestCredentials} [credentials='omit'] - Request credential. Defaults to 'omit'.
 * @param {'GET' | 'POST'} [requestMethod='GET'] - Request method. Defaults to 'GET'.
 * * @param {RequestMode | undefined} [mode='cors'] - Request mode. Defaults to 'GET'.
 */
export const getRequests = async (
	sveltekitFetch,
	targetUrl,
	credentials = 'omit',
	requestMethod = 'GET',
	mode = 'cors'
) => {
	const headers = { 'Content-Type': 'application/json' };

	const requestInitOptions = {
		method: requestMethod,
		mode: mode,
		credentials: credentials,
		headers: headers
	};

	const res = await sveltekitFetch(targetUrl, requestInitOptions);

	return res.ok && (await res.json());
};

/**
 * Get coin prices.
 * @file lib/utils/helpers.js
 * @param {typeof fetch} sveltekitFetch - Fetch object from sveltekit
 * @param {string} tags - The tags of the coins to fetch prices for.
 * @param {string} currency - The currency to fetch prices in.
 */
export const getCoinsPricesServer = async (sveltekitFetch, tags, currency) => {
	const res = await getRequests(
		sveltekitFetch,
		`/api/crypto/prices?tags=${tags}&currency=${currency}`
	);

	return res;
};

/**
 * Format price to be more readable.
 * @file lib/utils/helpers.js
 * @param {number} price - The price to format.
 */
export function formatPrice(price) {
	return price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

/**
 * Format the coin name to be more readable.
 * @file lib/utils/helpers.js
 * @param {string} coinName - The coin name to format.
 * @param {string} coinSymbol - The coin symbol to format.
 */
export function formatCoinName(coinName, coinSymbol) {
	// Format the name by capitalizing the first letter of each word
	const formattedName = coinName.toLowerCase().replace(/(?:^|\s)\S/g, (a) => a.toUpperCase());

	// Return the formatted name with the coin's symbol (if available)
	return `${formattedName} (${coinSymbol.toUpperCase()})`;
}

export function timeAgo(dateString) {
	const date = new Date(dateString);
	const now = new Date();

	const secondsAgo = Math.round((now - date) / 1000);
	const minutesAgo = Math.round(secondsAgo / 60);
	const hoursAgo = Math.round(minutesAgo / 60);
	const daysAgo = Math.round(hoursAgo / 24);

	const rtf = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });

	if (secondsAgo < 60) {
		return rtf.format(-secondsAgo, 'second');
	} else if (minutesAgo < 60) {
		return rtf.format(-minutesAgo, 'minute');
	} else if (hoursAgo < 24) {
		return rtf.format(-hoursAgo, 'hour');
	} else if (daysAgo < 30) {
		return rtf.format(-daysAgo, 'day');
	} else {
		// Fallback to a more standard date format
		return date.toLocaleDateString();
	}
}

/**
 * @type {import('chart.js').ChartConfiguration<'line', { x: Date; y: number; }[], unknown>}
 */
export const chartConfig = {
	type: 'line',
	data: {
		datasets: []
	},
	options: {
		scales: {
			x: {
				type: 'time',
				time: {
					unit: 'day'
				},
				title: {
					display: true,
					text: 'Date'
				}
			},
			y: {
				title: {
					display: true,
					text: 'Value'
				}
			}
		},
		responsive: true,
		tooltips: {
			enabled: true,
			mode: 'single',
			callbacks: {
				label: function (tooltipItems, data) {
					return tooltipItems.yLabel + ' : ' + tooltipItems.xLabel;
				},
				title: function (tooltipItems, data) {
					return 'Custom Title';
				}
			},
			backgroundColor: '#FFF',
			titleFontSize: 16,
			titleFontColor: '#0066ff',
			bodyFontColor: '#000',
			bodyFontSize: 14,
			displayColors: false
		}
	}
};

/**
 * Handle zooming on a chart.
 * @file lib/utils/helpers.js
 * @param {WheelEvent} event - The event to handle
 * @param {Chart<"line", { x: Date; y: number; }[], unknown>} chart - The chart to zoom
 */
export const handleZoom = (event, chart) => {
	event.preventDefault();
	const zoomFactor = 1.1;
	const direction = event.deltaY > 0 ? 1 : -1;

	const { min, max } = chart.scales.x;
	const range = max - min;
	const newRange = direction > 0 ? range * zoomFactor : range / zoomFactor;
	const center = (event.offsetX / event.target.clientWidth) * (max - min) + min;
	const newMin = center - (center - min) * (newRange / range);
	const newMax = center + (max - center) * (newRange / range);

	chart.options.scales.x.min = newMin;
	chart.options.scales.x.max = newMax;

	chart.update();
};

/**
 * Highlight code blocks.
 * @file lib/utils/helpers.js
 * @param {import('highlight.js').HLJSApi} hljs - The highlight.js object
 */
export const highlightCodeBlocks = (hljs) => {
	const codeBlocks = document.querySelectorAll('pre code');
	codeBlocks.forEach((block) => {
		if (block.dataset.highlighted) {
			delete block.dataset.highlighted;
		}
		block.innerHTML = block.innerHTML; // Force a reflow
		hljs.highlightElement(block);
		block.dataset.highlighted = 'yes';
	});
};
