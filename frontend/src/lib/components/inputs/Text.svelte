<script>
	export let label, id, name, placeholder, value;
</script>

<label class="block text-sm font-bold mb-2" for={id}> {label} </label>
<input
	class="shadow appearance-none bg-black border border-[#145369] rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:border-[#2596be] text-gray-500"
	{id}
	{name}
	type="text"
	{placeholder}
	{value}
	required
/>
