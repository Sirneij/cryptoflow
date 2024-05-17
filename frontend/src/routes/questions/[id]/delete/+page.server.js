import { BASE_API_URI } from '$lib/utils/constants';
import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').Actions} */
export const actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	delete: async ({ fetch, cookies, params }) => {
		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'DELETE',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			}
		};

		const res = await fetch(`${BASE_API_URI}/qa/questions/${params.id}`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		redirect(302, `/`);
	}
};
