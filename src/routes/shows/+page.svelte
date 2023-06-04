<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Show } from '$lib/types';

	async function getShows(): Promise<Show[]> {
		const raw = await invoke<object[]>('list_shows').catch((err: string) => {
			// TODO: Actually send this to an error toast
			console.error(err);
		});

		let data = raw?.map((show) => {
			return new Show(show);
		});

		return data ?? [];
	}

	let shows_promise = getShows();
</script>

<h1 class="text-5xl">Shows</h1>
{#await shows_promise}
	<div class="placeholder animate-pulse" />
{:then shows}
	<div class="grid grid-rows-2 grid-flow-col gap-4 mt-4">
		{#each shows as show}
			<a href="/shows/{show.id}">
				<div class="rounded-lg p-4 card card-hover flex flex-col col-span-1 min-h-full">
					<h2 class="text-2xl font-bold dark:text-blue-300 text-cyan-700">{show.name}</h2>
					<p class="text-base">{show.start_date.toLocaleDateString()}</p>
					{#if show.location}
						<p>
							<span class="font-semibold text-md">Location: </span>{show.location}
						</p>
					{/if}
					{#if show.entry_deadline}
						<p>
							<span class="font-semibold text-md"
								>Entry Deadline:
							</span>{show.entry_deadline.toLocaleDateString()}
						</p>
					{/if}
				</div>
			</a>
		{/each}
	</div>
{/await}
