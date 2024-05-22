export const BASE_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_API_URI_DEV
	: import.meta.env.VITE_BASE_API_URI_PROD;

export const NUM_OF_COINS_TO_SHOW = 10;

export const PASSWORD_ERROR_MESSAGE =
	'Password must be at least 8 characters long and contain at least one letter and one number';
