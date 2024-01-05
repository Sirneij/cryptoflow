<script>
	import developerImage from '$lib/assets/logo.png';

	export let data;

	$: ({ questions, coins } = data);
</script>

<div class="min-h-screen flex flex-col md:flex-row text-[#efefef]">
	<!-- Left Column for Tags -->
	<div class="hidden md:block md:w-1/4 p-4">
		<!-- Developer Profile Card -->
		<div
			class="bg-[#041014] hover:bg-black border border-black hover:border-[#145369] rounded-lg shadow p-4 mb-4"
		>
			<img src={developerImage} alt="Developer" class="rounded-full w-24 h-24 mx-auto mb-3" />
			<h3 class="text-center text-xl font-bold mb-2">Your Name</h3>
			<p class="text-center">Developer & Creator of CryptoFlow</p>
		</div>
		<div
			class="bg-[#041014] p-6 rounded-lg shadow mb-6 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Tags</h2>
			<ul class="space-y-2">
				{#each coins as tag}
					<li class="border-b border-[#0a0a0a] hover:bg-[#041014] px-3 py-1">{tag.name}</li>
				{/each}
			</ul>
		</div>
	</div>

	<div class="md:w-1/2 p-4">
		{#if questions}
			{#each questions as question (question.id)}
				<div
					class="
				bg-[#041014] mb-4 rounded-lg shadow hover:bg-black border border-black hover:border-[#145369]"
				>
					<div class="p-4">
						<a href="/questions/{question.id}" class="text-xl font-semibold hover:text-[#2596be]">
							{question.title}
						</a>
						<!-- <p class="mt-2">{article.description}</p> -->
						<div class="mt-3 flex flex-wrap">
							{#each question.tags as tag}
								<span
									class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
								>
									{tag.name}
								</span>
							{/each}
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>

	<!-- Right Column for Charts -->
	<div class="hidden md:block md:w-1/4 p-4">
		<div
			class="bg-[#041014] rounded-lg shadow p-4 hover:bg-black border border-black hover:border-[#145369]"
		>
			<h2 class="text-xl font-semibold mb-4">Charts</h2>
			<!-- Insert your chart components or images here -->
		</div>
	</div>
</div>
