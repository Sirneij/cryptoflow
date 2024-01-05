import { BASE_API_URI } from '$lib/utils/constants';

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
