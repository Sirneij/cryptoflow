import { BASE_API_URI } from '$lib/utils/constants';
import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	// redirect user if logged in
	if (locals.user) {
		redirect(302, '/');
	}
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ request, fetch, params }) => {
		const data = await request.formData();
		let token = String(data.get('token'));

		token = token.replace(/[-\s]/g, '').trim();

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				id: params.id,
				token: token
			})
		};

		const res = await fetch(`${BASE_API_URI}/users/activate`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		const response = await res.json();
		redirect(302, `/users/login?message=${response.message}`);
	}
};
