<script lang="ts">
	import { page } from '$app/stores';
	import { Show } from '$lib/types';
	import { invoke } from '@tauri-apps/api/tauri';
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';
	import ClassesList from '$lib/ClassesList.svelte';

	let tabSet: number = 0;

	async function getShow(id: string): Promise<Show> {
		const data = await invoke<object>('get_show', { showId: parseInt(id, 10) })
			.catch((err: string) => {
				// TODO: Actually send this to an error toast
				console.error(err);
			})
			.then((res: void | object) => {
				if (res instanceof Object) {
					return new Show(res);
				}
				return new Show(Object());
			});

		return data;
	}

	let show_promise = getShow($page.params.id);
</script>

{#await show_promise}
	<div class="placeholder animate-pulse" />
{:then show}
	<div class="grid grid-cols-2 grid-flow-colborder-solid border-b-4 mb-2">
		<div><h1 class="text-4xl">{show.name}</h1></div>
		<div class="relative">
			<p class="text-lg text-right align-text-bottom m-0 p-0 absolute inset-x-0 bottom-0">
				{new Date(show.start_date).toLocaleDateString()}
			</p>
		</div>
	</div>
	<TabGroup>
		<Tab bind:group={tabSet} name="tab1" value={0}>Divisions</Tab>
		<Tab bind:group={tabSet} name="tab2" value={1}>Entries</Tab>
		<Tab bind:group={tabSet} name="tab2" value={2}>Results</Tab>
		<!-- Tab Panels --->
		<svelte:fragment slot="panel">
			{#if tabSet === 0}
				<ClassesList />
			{:else if tabSet === 1}
				(tab panel 2 contents)
			{:else if tabSet === 2}
				(tab panel 3 contents)
			{/if}
		</svelte:fragment>
	</TabGroup>
{/await}
