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

const coinSymbols = {
	Bitcoin: 'BTC',
	Ethereum: 'ETH'
	// Add other coins and their symbols here
};

/**
 * Format the coin name to be more readable.
 * @file lib/utils/helpers.js
 * @param {string} coinName - The coin name to format.
 */
export function formatCoinName(coinName) {
	// Format the name by capitalizing the first letter of each word
	const formattedName = coinName.toLowerCase().replace(/(?:^|\s)\S/g, (a) => a.toUpperCase());

	// Return the formatted name with the coin's symbol (if available)
	return `${formattedName} (${coinSymbols[formattedName] || 'N/A'})`;
}
