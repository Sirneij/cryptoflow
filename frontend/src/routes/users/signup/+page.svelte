<script>
	import Logo from '$lib/assets/logo.png';
	// Example data. Replace with your actual data.
	let question = {
		title: 'How to integrate Tailwind CSS with Svelte?',
		content: "I'm trying to integrate Tailwind CSS in my Svelte project but facing some issues...",
		tags: ['Svelte', 'TailwindCSS', 'Web Development'],
		author: {
			name: 'John Doe',
			image: Logo // Replace with actual image path
		},
		created_at: 'Jan 02, 2024',
		updated_at: 'Jan 11, 2024'
	};

	let answers = [
		{
			id: 1,
			content: 'First, install Tailwind CSS via npm...',
			author: {
				name: 'Jane Smith',
				image: Logo // Replace with actual image path
			}
		},
		{
			id: 1,
			content: 'First, install Tailwind CSS via npm...',
			author: {
				name: 'Jane Smith',
				image: Logo // Replace with actual image path
			}
		},
		{
			id: 1,
			content: 'First, install Tailwind CSS via npm...',
			author: {
				name: 'Jane Smith',
				image: Logo // Replace with actual image path
			}
		}
	];

	let relatedQuestions = [
		{ id: 1, title: 'Related Question 1' }
		// More related questions...
	];

	function addAnswer() {
		// Logic to add an answer
	}
</script>

<div class="max-w-5xl mx-auto p-4">
	<!-- Stats Section -->
	<div class="bg-[#0a0a0a] p-6 rounded-lg shadow mb-6 flex justify-between items-center">
		<p>Created: {question.created_at}</p>
		<p>Last Updated: {question.updated_at}</p>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-12 gap-4">
		<!-- Main Content -->
		<div class="md:col-span-9">
			<!-- Question Section -->
			<div class="bg-[#041014] p-6 rounded-lg shadow mb-6 border border-black">
				<h1 class="text-2xl font-bold mb-4">{question.title}</h1>
				<p>{question.content}</p>
				<div class="flex mt-4">
					{#each question.tags as tag}
						<span
							class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
						>
							{tag}
						</span>
					{/each}
				</div>
				<div class="flex justify-end mt-4">
					<button class="mr-2 text-blue-500 hover:text-blue-600">Edit</button>
					<button class="mr-2 text-red-500 hover:text-red-600">Delete</button>
					<button class="text-yellow-500 hover:text-yellow-600">Flag</button>
				</div>
				<hr class="my-4" />
				<div class="flex justify-end items-center">
					<span class="mr-3">{question.author.name}</span>
					<img
						src={question.author.image}
						alt={question.author.name}
						class="h-10 w-10 rounded-full"
					/>
				</div>
			</div>

			<!-- Answers Section -->
			<h2 class="text-xl font-bold mb-4">Answers</h2>

			<!-- Answers Section -->
			{#each answers as answer (answer.id)}
				<div class="bg-[#041014] p-6 rounded-lg shadow mb-4">
					<p>{answer.content}</p>
					<hr class="my-4" />
					<div class="flex justify-end items-center">
						<span class="mr-3">{answer.author.name}</span>
						<img
							src={answer.author.image}
							alt={answer.author.name}
							class="h-10 w-10 rounded-full"
						/>
					</div>
				</div>
				<div class="flex justify-end mt-4">
					<button class="mr-2 text-blue-500 hover:text-blue-600">Edit</button>
					<button class="mr-2 text-red-500 hover:text-red-600">Delete</button>
					<button class="text-yellow-500 hover:text-yellow-600">Flag</button>
				</div>
			{/each}

			<!-- Post Answer Section -->
			<div class="bg-[#041014] p-6 rounded-lg shadow">
				<h2 class="text-xl font-bold mb-4">Your Answer</h2>
				<textarea
					class="w-full p-4 text-gray-700 border rounded"
					rows="6"
					placeholder="Write your answer here..."
				></textarea>
				<button
					class="mt-4 px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
					on:click={addAnswer}>Post Your Answer</button
				>
			</div>
		</div>

		<!-- Right Sidebar -->
		<div class="md:col-span-3">
			<h2 class="text-xl font-semibold mb-4">Related Questions</h2>
			{#each relatedQuestions as question (question.id)}
				<div
					class="bg-[#041014] p-4 rounded-lg shadow mb-4 hover:bg-[#0a0a0a] border border-[#041014] hover:border-[#041014]"
				>
					<a href="/questions/{question.id}" class="hover:text-[#2596be]">{question.title}</a>
				</div>
			{/each}
		</div>
	</div>
</div>
