import { BASE_API_URI, PASSWORD_ERROR_MESSAGE } from '$lib/utils/constants';
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
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	register: async ({ request, fetch }) => {
		const data = await request.formData();
		const firstName = String(data.get('first_name'));
		const lastName = String(data.get('last_name'));
		const email = String(data.get('email'));
		const password = String(data.get('password'));
		const confirmPassword = String(data.get('confirm_password'));

		// Check if first name is empty
		if (!firstName) {
			const errors = [{ id: 1, message: 'First name is required' }];
			return fail(400, { errors: errors });
		}

		// Check if last name is empty
		if (!lastName) {
			const errors = [{ id: 1, message: 'Last name is required' }];
			return fail(400, { errors: errors });
		}

		// Check if email is empty
		if (!email) {
			const errors = [{ id: 1, message: 'Email is required' }];
			return fail(400, { errors: errors });
		}

		// Check if password is empty
		if (!password) {
			const errors = [{ id: 1, message: 'Password is required' }];
			return fail(400, { errors: errors });
		}

		// Check if password is valid
		if (password.length < 8 || !/[A-Za-z]/.test(password) || !/[0-9]/.test(password)) {
			const errors = [
				{
					id: 1,
					message: PASSWORD_ERROR_MESSAGE
				}
			];
			return fail(400, { errors: errors });
		}

		if (password !== confirmPassword) {
			const errors = [{ id: 1, message: 'Passwords do not match' }];
			return fail(400, { errors: errors });
		}

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email: email,
				password: password,
				first_name: firstName,
				last_name: lastName
			})
		};

		const res = await fetch(`${BASE_API_URI}/users/register`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		redirect(302, `/users/activate/${response.user_id}?message=${response.message}`);
	}
};
