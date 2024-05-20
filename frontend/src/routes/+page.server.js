import { BASE_API_URI } from '$lib/utils/constants';
import { fail } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch }) {
	const fetchQuestions = async () => {
		const res = await fetch(`${BASE_API_URI}/qa/questions`);
		return res.ok && (await res.json());
	};

	const fetchCoins = async () => {
		const res = await fetch(`${BASE_API_URI}/crypto/coins`);
		return res.ok && (await res.json());
	};

	const questions = await fetchQuestions();
	const coins = await fetchCoins();

	return {
		questions,
		coins
	};
}

// Get coin data form action

/** @type {import('./$types').Actions} */
export const actions = {
	/**
	 * Get coin market history data from the API
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	getCoinData: async ({ request, fetch }) => {
		const data = await request.formData();
		const coinIDs = String(data.get('tags'));
		const days = Number(data.get('days'));
		const res = await fetch(
			`${BASE_API_URI}/crypto/coin_prices?tags=${coinIDs}&currency=USD&days=${days}`
		);
		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		return {
			status: 200,
			marketData: response
		};
	}
};
