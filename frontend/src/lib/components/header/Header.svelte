<script>
	import Logo from '$lib/assets/logo.png';
</script>

<nav class="shadow py-4 text-[#9b9b9b]">
	<div class="max-w-7xl mx-auto px-4">
		<div class="flex justify-between">
			<div class="flex space-x-4">
				<!-- Logo -->
				<div>
					<a href="/" class="flex items-center py-2 px-3">
						<img src={Logo} alt="Logo" class="h-8 w-8 mr-2" />
						<span class="font-bold">CryptoFlow</span>
					</a>
				</div>
			</div>
			<div class="hidden md:flex items-center space-x-1">
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
		</div>
	</div>
</nav>
