import { BASE_API_URI } from '$lib/utils/constants';
import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch, locals }) {
	if (!locals.user) {
		redirect(302, `/users/login?next=/questions/ask`);
	}
	const fetchCoins = async () => {
		const res = await fetch(`${BASE_API_URI}/crypto/coins`);
		return res.ok && (await res.json());
	};
	const coins = await fetchCoins();

	return {
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
	ask: async ({ request, fetch, cookies }) => {
		const data = await request.formData();
		const title = String(data.get('title'));
		const content = String(data.get('content'));
		const tags = String(data.get('tags'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
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

		const res = await fetch(`${BASE_API_URI}/qa/ask`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		redirect(302, `/questions/${response.id}`);
	}
};
