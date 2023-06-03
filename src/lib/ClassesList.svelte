<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Division } from './types';
	import ChevronLeft20Solid from '~icons/heroicons/chevron-left-20-solid';
	import ChevronDown20Solid from '~icons/heroicons/chevron-down-20-solid';
	import ClassesTable from './ClassesTable.svelte';

	let selected_id: null | number = null;

	function select_row(id: number): void {
		if (id !== selected_id) {
			selected_id = id;
		} else {
			selected_id = null;
		}
	}

	function select_row_with_key(event: KeyboardEvent, id: number): void {
		if (event.key === 'Enter') {
			select_row(id);
		}
	}

	async function getDivisions(): Promise<Division[]> {
		let raw = await invoke<object[]>('list_divisions').catch((err: string) => {
			// TODO: Actually send this to an error toast
			console.error(err);
		});

		let data = raw?.map((div) => {
			return new Division(div);
		});

		return data ?? [];
	}

	let division_promise = getDivisions();
</script>

{#await division_promise}
	<!-- TODO: Real loading -->
	<p>...loading</p>
{:then divisions}
	<div class="w-full p-4 space-y-4 card">
		<dl class="list-dl">
			{#each divisions as division}
				<div
					class="border-b border-slate-300 cursor-pointer hover:bg-slate-600"
					class:bg-slate-600={selected_id === division.id}
					on:click={() => select_row(division.id)}
					on:keydown={(event) => select_row_with_key(event, division.id)}
				>
					<span class="flex-auto">
						<dt>
							{division.name}
						</dt>
					</span>
					<span>
						{#if selected_id === division.id}
							<ChevronDown20Solid />
						{:else}
							<ChevronLeft20Solid />
						{/if}
					</span>
				</div>
				{#if selected_id === division.id}
					<div class="card">
						<dd class="w-full">
							<h3 class="text-xl font-bold">Classes</h3>
							<ClassesTable division_id={division.id} />
						</dd>
					</div>
				{/if}
			{/each}
		</dl>
	</div>
{/await}
