
interface User {
	email: string;
	first_name: string;
	last_name: string;
	id: string;
	is_staff: boolean;
	is_active: boolean;
	thumbnail: string;
	is_superuser: boolean;
	date_joined: string;
}

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: User;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
