<script>
	// Example data structure. Replace with your actual data source.
	let articles = [
		{
			id: 1,
			title: '5 Tips for Starting with Web Development',
			description: 'A quick guide to kick-start your web development journey.',
			tags: ['Beginner', 'HTML', 'CSS']
		},
		{
			id: 2,
			title: 'Understanding Async/Await in JavaScript',
			description: 'Deep dive into asynchronous programming in JavaScript.',
			tags: ['JavaScript', 'Async', 'Code']
		}
		// Add more articles as needed
	];
	let tags = ['JavaScript', 'WebDev', 'React', 'CSS', 'HTML', 'Svelte'];
</script>

<div class="min-h-screen flex flex-col md:flex-row text-[#efefef]">
	<!-- Left Column for Tags -->
	<div class="md:w-1/4 p-4">
		<div
			class="bg-[#041014] p-6 rounded-lg shadow mb-6 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Tags</h2>
			<ul class="space-y-2">
				{#each tags as tag}
					<li class="border-b border-[#0a0a0a] hover:bg-black px-3 py-1">{tag}</li>
				{/each}
			</ul>
		</div>
	</div>

	<!-- Middle Column for Articles -->
	<div class="md:w-1/2 p-4">
		{#each articles as article (article.id)}
			<div
				class="
				bg-[#041014] mb-4 rounded-lg shadow hover:bg-black border border-black hover:border-[#145369]"
			>
				<div class="p-4">
					<a href="/article/{article.id}" class="text-xl font-semibold hover:text-[#2596be]">
						{article.title}
					</a>
					<p class="mt-2">{article.description}</p>
					<div class="mt-3 flex flex-wrap">
						{#each article.tags as tag}
							<span
								class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
							>
								{tag}
							</span>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	</div>

	<!-- Right Column for Charts -->
	<div class="md:w-1/4 p-4">
		<div
			class="bg-[#041014] rounded-lg shadow p-4 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Charts</h2>
			<!-- Insert your chart components or images here -->
		</div>
	</div>
</div>
