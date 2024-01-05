<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import Logo from '$lib/assets/logo.png';

	let showDropdown = false;

	function toggleDropdown() {
		showDropdown = !showDropdown;
	}

	function closeDropdown() {
		showDropdown = false;
	}
</script>

<nav class="shadow py-4 text-[#9b9b9b]">
	<div class="max-w-7xl mx-auto px-4">
		<div class="flex justify-between">
			<!-- Logo -->
			<div class="flex space-x-4">
				<a href="/" class="flex items-center py-2 px-3">
					<img src={Logo} alt="Logo" class="h-8 w-8 mr-2" />
					<span class="font-bold">CryptoFlow</span>
				</a>
			</div>

			<!-- Right side items -->
			<div class="flex items-center space-x-4">
				{#if !$page.data.user}
					<div class="hidden md:flex space-x-1">
						<a
							href="/users/login"
							class="py-2 px-3 hover:text-white transition duration-300 ease-in-out"
						>
							Login
						</a>
						<a
							href="/users/signup"
							class="py-2 px-3 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-[#efefef] hover:text-white transition duration-300 ease-in-out"
						>
							Sign Up
						</a>
					</div>
				{:else}
					<!-- User Image and Dropdown -->
					<div class="relative">
						<button on:click|stopPropagation={toggleDropdown} class="focus:outline-none">
							<img src={Logo} alt="User" class="h-10 w-10 rounded-full" />
						</button>
						{#if showDropdown}
							<div
								class="absolute right-0 mt-2 w-48 bg-[#041014] rounded-lg shadow-xl"
								on:click|stopPropagation
								role="none"
							>
								<a
									href="/"
									class="block px-4 py-2 text-sm hover:bg-gray-700"
									on:click={closeDropdown}
								>
									Profile
								</a>
								<a
									href="/"
									class="block px-4 py-2 text-sm hover:bg-gray-700"
									on:click={closeDropdown}
								>
									Settings
								</a>
								<form
									class="block px-4 py-2 text-sm hover:bg-gray-700"
									action="/users/login?/logout"
									method="POST"
									use:enhance={async () => {
										return async ({ result }) => {
											await applyAction(result);
										};
									}}
								>
									<input type="hidden" name="next" value={$page.url.pathname} />
									<button>Logout</button>
								</form>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
</nav>

<!-- Global click handler to close dropdown -->
<svelte:window on:click={closeDropdown} />
