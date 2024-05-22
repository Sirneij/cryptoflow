<script>
	import { applyAction, enhance } from '$app/forms';
	import { page } from '$app/stores';
	import Logo from '$lib/assets/logo.png';
	import {
		formatCoinName,
		formatPrice,
		getCoinsPricesServer,
		rehighlightCodeBlocks,
		timeAgo
	} from '$lib/utils/helpers.js';
	import { onMount } from 'svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { scale } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import Modal from '$lib/components/Modal.svelte';
	import hljs from 'highlight.js';
	import ShowError from '$lib/components/ShowError.svelte';
	import { notification } from '$lib/stores/notification.store.js';
	import 'highlight.js/styles/night-owl.css';

	export let data;

	/** @type {import('./$types').ActionData} */
	export let form;
	/** @type {Array<{"name": String, "price": number}>} */
	let coinPrices = [];
	let processing = false;
	let showDeleteModal = false;
	let showEditModal = false;
	let answerID = '';
	let answerContent = '';

	$: ({ question, answers } = data);

	const openModal = (isDelete = true) => {
		if (isDelete) {
			showDeleteModal = true;
		} else {
			showEditModal = true;
		}
	};

	const closeModal = () => {
		showDeleteModal = false;
		showEditModal = false;
	};

	/** @param {String} id */
	const setAnswerID = (id) => (answerID = id);
	/** @param {String} content */
	const setAnswerContent = (content) => (answerContent = content);

	onMount(async () => {
		hljs.highlightAll();
		if (question) {
			const tagsString = question.tags
				.map(
					/** @param {{"id": String}} tag */
					(tag) => tag.id
				)
				.join(',');
			coinPrices = await getCoinsPricesServer($page.data.fetch, tagsString, 'usd');
		}
	});

	/** @type {import('./$types').SubmitFunction} */
	const handleAnswerQuestion = async () => {
		processing = true;
		return async ({ result }) => {
			processing = false;
			if (result.type === 'success') {
				if (result.data && 'answer' in result.data) {
					answers = [...answers, result.data.answer];
					rehighlightCodeBlocks(hljs);
					notification.set({ message: 'Answer posted successfully', colorName: 'blue' });
				}
			}
			await applyAction(result);
		};
	};

	/** @type {import('./$types').SubmitFunction} */
	const handleDeleteAnswer = async () => {
		return async ({ result }) => {
			closeModal();
			if (result.type === 'success') {
				answers = answers.filter(
					/** @param {{"id": String}} answer */
					(answer) => answer.id !== answerID
				);
				notification.set({ message: 'Answer deleted successfully', colorName: 'blue' });
			}
			await applyAction(result);
		};
	};

	/** @type {import('./$types').SubmitFunction} */
	const handleUpdateAnswer = async () => {
		return async ({ result }) => {
			closeModal();
			if (result.type === 'success') {
				answers = answers.map(
					/** @param {{"id": String}} answer */
					(answer) => {
						if (result.data && 'answer' in result.data) {
							return answer.id === answerID ? result.data.answer : answer;
						}
						return answer;
					}
				);
				answerContent = '';
				rehighlightCodeBlocks(hljs);
				notification.set({ message: 'Answer updated successfully', colorName: 'blue' });
			}
			await applyAction(result);
		};
	};
</script>

<div class="max-w-5xl mx-auto p-4">
	<!-- Stats Section -->
	<div class="bg-[#0a0a0a] p-6 rounded-lg shadow mb-6 flex justify-between items-center">
		<p>Asked: {timeAgo(question.created_at)}</p>
		<p>Modified: {timeAgo(question.updated_at)}</p>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-12 gap-4">
		<!-- Main Content -->
		<div class="md:col-span-9">
			<!-- Question Section -->
			<div class="bg-[#041014] p-6 rounded-lg shadow mb-6 border border-black">
				<h1 class="text-2xl font-bold mb-4">{question.title}</h1>
				<p>{@html question.content}</p>
				<div class="flex mt-4 flex-wrap">
					{#each question.tags as tag}
						<span
							class="mr-2 mb-2 px-3 py-1 text-sm bg-[#041014] border border-[#145369] hover:border-[#2596be] rounded"
						>
							{tag.name.toLowerCase()}
						</span>
					{/each}
				</div>
				<div class="flex justify-end mt-4">
					{#if $page.data.user && question.author.id === $page.data.user.id}
						<a
							class="mr-2 text-blue-500 hover:text-blue-600"
							href="/questions/{question.id}/update"
						>
							Edit
						</a>
						<a class="mr-2 text-red-500 hover:text-red-600" href="/questions/{question.id}/delete">
							Delete
						</a>
					{/if}
				</div>
				<hr class="my-4" />
				<div class="flex justify-end items-center">
					<span class="mr-3">
						{question.author.first_name + ' ' + question.author.last_name}
					</span>
					<img
						src={question.author.thumbnail ? question.author.thumbnail : Logo}
						alt={question.author.first_name + ' ' + question.author.last_name}
						class="h-10 w-10 rounded-full"
					/>
				</div>
			</div>

			<!-- Answers Section -->
			<h2 class="text-xl font-bold mb-4">Answers</h2>
			{#each answers as answer (answer.id)}
				<div
					class="bg-[#041014] p-6 rounded-lg shadow mb-4"
					transition:scale|local={{ start: 0.4 }}
					animate:flip={{ duration: 200 }}
				>
					<p>{@html answer.content}</p>

					<div class="flex justify-end mt-4">
						{#if $page.data.user && answer.author.id === $page.data.user.id}
							<button
								class="mr-2 text-blue-500 hover:text-blue-600"
								on:click={() => {
									openModal(false);
									setAnswerID(answer.id);
									setAnswerContent(answer.raw_content);
								}}
							>
								Edit
							</button>
							<button
								class="mr-2 text-red-500 hover:text-red-600"
								on:click={() => {
									openModal();
									setAnswerID(answer.id);
								}}
							>
								Delete
							</button>
						{/if}
					</div>
					<hr class="my-4" />
					<div class="flex justify-end items-center">
						<span class="mr-3">{answer.author.first_name + ' ' + answer.author.last_name}</span>
						<img
							src={answer.author.thumbnail ? answer.author.thumbnail : Logo}
							alt={answer.author.first_name + ' ' + answer.author.last_name}
							class="h-10 w-10 rounded-full"
						/>
					</div>
				</div>
			{:else}
				<div class="bg-[#041014] p-6 rounded-lg shadow mb-4">
					<p>No answers yet.</p>
				</div>
			{/each}

			<!-- Post Answer Section -->
			<form
				class="bg-[#041014] p-6 rounded-lg shadow"
				method="POST"
				action="?/answer"
				use:enhance={handleAnswerQuestion}
			>
				<h2 class="text-xl font-bold mb-4">Your Answer</h2>
				<ShowError {form} />
				<textarea
					class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:border-[#2596be] focus:outline-none"
					rows="6"
					bind:value={answerContent}
					name="content"
					placeholder="Write your answer here (markdown supported)..."
				/>

				{#if processing}
					<Loader width={20} message="Posting your answer..." />
				{:else}
					<button
						class="mt-4 px-6 py-2 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-white rounded"
					>
						{#if $page.data.user && $page.data.user.id === question.author.id}
							Answer your question
						{:else}
							Post Your Answer
						{/if}
					</button>
				{/if}
			</form>
		</div>

		<!-- Right Sidebar -->
		<div class="md:col-span-3">
			<h2 class="text-xl font-semibold mb-4">Current prices</h2>
			<div
				class="bg-[#041014] rounded-lg shadow p-4 hover:bg-black border border-black hover:border-[#145369]"
			>
				<div class="space-y-4">
					{#each coinPrices as coin (coin.name)}
						<div
							class="bg-[#145369] p-3 rounded-lg text-center"
							transition:scale|local={{ start: 0.4 }}
							animate:flip={{ duration: 200 }}
						>
							<p class="text-3xl font-bold">
								<span class="text-base">$</span>{formatPrice(coin.price)}
							</p>
							{#if question.tags.find(/** @param {{"id": String}} tag */ (tag) => tag.id === coin.name)}
								<div class="flex items-center text-lg">
									<img
										class="w-8 h-8 rounded-full mr-2 transition-transform duration-500 ease-in-out transform hover:rotate-180"
										src={question.tags.find(
											/** @param {{"id": String}} tag */
											(tag) => tag.id === coin.name
										).image}
										alt={coin.name}
									/>
									<span class="mr-2">
										{formatCoinName(
											coin.name,
											question.tags.find(
												/** @param {{"id": String}} tag */
												(tag) => tag.id === coin.name
											).symbol
										)}
									</span>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		</div>
	</div>
</div>

{#if showDeleteModal}
	<Modal on:close={closeModal}>
		<form
			class="bg-[#041014] p-6 rounded-lg shadow text-center"
			method="POST"
			action="?/deleteAnswer"
			use:enhance={handleDeleteAnswer}
		>
			<ShowError {form} />
			<p class="text-red-500 p-3 mb-4 italic">
				Are you sure you want to delete this answer (id={answerID})
			</p>
			<input type="hidden" name="answerID" value={answerID} />
			<button
				class="mt-4 px-6 py-2 bg-[#041014] border border-red-400 hover:border-red-700 text-red-600 rounded"
			>
				Delete Answer
			</button>
		</form>
	</Modal>
{/if}

{#if showEditModal}
	<Modal on:close={closeModal}>
		<form
			class="bg-[#041014] p-6 rounded-lg shadow text-center"
			method="POST"
			action="?/updateAnswer"
			use:enhance={handleUpdateAnswer}
		>
			<ShowError {form} />
			<input type="hidden" name="answerID" value={answerID} />
			<textarea
				class="w-full p-4 bg-[#0a0a0a] text-[#efefef] border border-[#145369] rounded focus:border-[#2596be] focus:outline-none"
				rows="6"
				bind:value={answerContent}
				name="content"
				placeholder="Write your answer here (markdown supported)..."
			/>
			<button
				class="mt-4 px-6 py-2 bg-[#041014] border border-[#145369] hover:border-[#2596be] text-white rounded"
			>
				Update Answer
			</button>
		</form>
	</Modal>
{/if}
