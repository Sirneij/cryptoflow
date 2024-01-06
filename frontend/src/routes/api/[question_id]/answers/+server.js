import { BASE_API_URI } from '$lib/utils/constants';
import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET({ fetch, params }) {
	const apiURL = `${BASE_API_URI}/qa/questions/${params.question_id}/answers`;
	const res = await fetch(apiURL);
	const data = res.ok && (await res.json());
	return json(data, { status: res.status });
}
