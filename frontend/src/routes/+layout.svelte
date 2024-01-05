<script>
	import '../app.css';
	import Header from '$lib/components/header/Header.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';
</script>

<header class="sticky top-0 z-50 bg-[#0a0a0a]">
	<Header />
</header>

<slot />

<Footer />
