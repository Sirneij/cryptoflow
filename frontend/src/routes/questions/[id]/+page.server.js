import { BASE_API_URI } from '$lib/utils/constants';
import { fail } from '@sveltejs/kit';

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

/** @type {import('./$types').Actions} */
export const actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	answer: async ({ request, fetch, params, cookies }) => {
		const data = await request.formData();
		const content = String(data.get('content'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			},
			body: JSON.stringify({
				content: content
			})
		};

		const res = await fetch(`${BASE_API_URI}/qa/answer/${params.id}`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		return {
			status: 200,
			answer: response
		};
	},
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	deleteAnswer: async ({ request, fetch, cookies }) => {
		const data = await request.formData();
		const answerID = String(data.get('answerID'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'DELETE',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			}
		};

		const res = await fetch(`${BASE_API_URI}/qa/answers/${answerID}`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		return {
			status: res.status
		};
	},
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	updateAnswer: async ({ request, fetch, cookies }) => {
		const data = await request.formData();
		const answerID = String(data.get('answerID'));
		const content = String(data.get('content'));

		/** @type {RequestInit} */
		const requestInitOptions = {
			method: 'PATCH',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('cryptoflow-sessionid')}`
			},
			body: JSON.stringify({
				content: content
			})
		};

		const res = await fetch(`${BASE_API_URI}/qa/answers/${answerID}`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors = [{ id: 1, message: response.message }];
			return fail(400, { errors: errors });
		}

		return {
			status: res.status,
			answer: await res.json()
		};
	}
};
