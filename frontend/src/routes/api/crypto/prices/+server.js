import { BASE_API_URI } from '$lib/utils/constants';
import { json } from '@sveltejs/kit';

export async function GET({ url }) {
	try {
		const apiURL = `${BASE_API_URI}/crypto/prices${url.search}`;
		const res = await fetch(apiURL);

		if (!res.ok) {
			// Handle error response from the API
			return json({ error: 'Failed to fetch data' }, { status: res.status });
		}

		const data = await res.json();
		return json(data, { status: res.status });
	} catch (error) {
		// Handle any other errors
		return json({ error: 'An error occurred' }, { status: 500 });
	}
}
