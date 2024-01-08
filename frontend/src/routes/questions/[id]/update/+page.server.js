import { BASE_API_URI } from '$lib/utils/constants';
import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch, locals, params }) {
	if (!locals.user) {
		throw redirect(302, `/users/login?next=/questions/${params.id}/update}`);
	}

	const fetchQuestion = async () => {
		const res = await fetch(`${BASE_API_URI}/qa/questions/${params.id}`);
		return res.ok && (await res.json());
	};
	const question = await fetchQuestion();

	const fetchCoins = async () => {
		const res = await fetch(`${BASE_API_URI}/crypto/coins`);
		return res.ok && (await res.json());
	};
	const coins = await fetchCoins();

	return {
		question,
		coins
	};
}

/** @type {import('./$types').Actions} */
export const actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	update: async ({ request, fetch, params, cookies }) => {
		const data = await request.formData();
		const title = String(data.get('title'));
		const content = String(data.get('content'));
		const tags = String(data.get('tags'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'PATCH',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			},
			body: JSON.stringify({
				title: title,
				content: content,
				tags: tags
			})
		};

		const res = await fetch(`${BASE_API_URI}/qa/questions/${params.id}`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		throw redirect(302, `/questions/${params.id}`);
	}
};
