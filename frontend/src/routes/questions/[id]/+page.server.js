import { BASE_API_URI } from '$lib/utils/constants';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch, params }) {
	const fetchQuestion = async () => {
		const res = await fetch(`${BASE_API_URI}/qa/questions/${params.id}`);
		return res.ok && (await res.json());
	};

	const fetchAnswers = async () => {
		const res = await fetch(`${BASE_API_URI}/qa/questions/${params.id}/answers`);
		return res.ok && (await res.json());
	};

	return {
		question: await fetchQuestion(),
		answers: await fetchAnswers()
	};
}
