<script>
	export let label,
		id,
		name,
		placeholder,
		/** @type {string} */
		value;
</script>

<div class="mb-2">
	<label for={id} class="block text-[#efefef] text-sm font-bold mb-2">{label}</label>
	<textarea
		{id}
		{name}
		rows="8"
		class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:outline-none focus:border-[#2596be] text-gray-500"
		{placeholder}
		bind:value
	></textarea>
</div>
