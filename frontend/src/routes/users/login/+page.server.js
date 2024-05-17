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
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @param cookies - SvelteKit's cookie object
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	login: async ({ request, fetch, cookies }) => {
		const data = await request.formData();
		const email = String(data.get('email'));
		const password = String(data.get('password'));
		const next = String(data.get('next'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email: email,
				password: password
			})
		};

		const res = await fetch(`${BASE_API_URI}/users/login`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		if (res.headers.has('Set-Cookie')) {
			const responseHeaders = Object.fromEntries(res.headers);
			const cookieString = responseHeaders['set-cookie'];

			const regexSessionId = /sessionid=([^;]*)/;
			const regexSameSite = /SameSite=([^;]*)/;
			const regexPath = /Path=([^;]*)/;
			const regexMaxAge = /Max-Age=([^;]*)/;

			const sessionID = (cookieString.match(regexSessionId) || [])[1];
			const sameSite = (cookieString.match(regexSameSite) || [])[1];
			const path = (cookieString.match(regexPath) || [])[1];
			const maxAge = (cookieString.match(regexMaxAge) || [])[1];

			/** @type {"lax" | "strict" | "none" | undefined} */
			let sameSiteValue;
			if (sameSite === 'Strict') {
				sameSiteValue = 'strict';
			} else if (sameSite === 'Lax') {
				sameSiteValue = 'lax';
			} else if (sameSite === 'None') {
				sameSiteValue = 'none';
			} else {
				sameSiteValue = undefined;
			}

			cookies.set('cryptoflow-sessionid', sessionID, {
				httpOnly: true,
				sameSite: sameSiteValue,
				path: path,
				secure: true,
				maxAge: maxAge !== undefined ? parseInt(maxAge) : undefined
			});
		}

		redirect(
			303,
			next !== 'null' && next !== null && next !== undefined && next !== 'undefined' ? next : '/'
		);
	},
	logout: async ({ fetch, cookies, request }) => {
		const data = await request.formData();
		const next = String(data.get('next'));
		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			}
		};

		const res = await fetch(`${BASE_API_URI}/users/logout`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [];
			errors.push({ id: 0, message: response.message });
			return fail(400, { errors: errors });
		}

		// eat the cookie
		cookies.delete('cryptoflow-sessionid', { path: '/' });

		// redirect the user
		redirect(
			302,
			next !== 'null' && next !== null && next !== undefined && next !== 'undefined'
				? next
				: '/users/login'
		);
	}
};
