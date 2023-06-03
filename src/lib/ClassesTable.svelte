<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import {
		createSvelteTable,
		flexRender as flexRenderOrig,
		getCoreRowModel,
		type Table
	} from '@tanstack/svelte-table';
	import type { ColumnDef, TableOptions } from '@tanstack/table-core/src/types';
	import type { Readable } from 'svelte/store';
	import type { ComponentType, SvelteComponentTyped } from 'svelte/internal';
	import { Class } from '$lib/types';

	// Workaround for https://github.com/TanStack/table/issues/4778
	const flexRender = <P extends Record<string, any>, C = any>(
		component: C,
		props: P
	): ComponentType<SvelteComponentTyped> =>
		flexRenderOrig(component, props) as ComponentType<SvelteComponentTyped>;

	export let division_id: number;

	const columns: ColumnDef<Class>[] = [
		{
			accessorKey: 'name',
			cell: (info) => info.getValue(),
			header: () => 'Name'
		}
	];

	let table_data: Readable<Table<Class>>;

	async function getClasses(): Promise<void> {
		let raw = await invoke<object[]>('list_classes_for_division', {
			division: division_id
		}).catch((err: string) => {
			// TODO: Actually send this to an error toast
			console.error(err);
		});

		let data = raw?.map((cls) => {
			return new Class(cls);
		});

		let options: TableOptions<Class> = {
			data: data ?? [],
			columns: columns,
			getCoreRowModel: getCoreRowModel()
		};

		table_data = createSvelteTable(options);
	}

	let table_promise = getClasses();
</script>

{#await table_promise}
	<!-- TODO: Real loading -->
	<p>...loading</p>
{:then}
	<div class="table-container">
		<table class="table table-hover table-interactive table-auto w-full">
			<thead>
				{#each $table_data.getHeaderGroups() as headerGroup}
					<tr>
						{#each headerGroup.headers as header}
							<th>
								{#if !header.isPlaceholder}
									<svelte:component
										this={flexRender(header.column.columnDef.header, header.getContext())}
									/>
								{/if}
							</th>
						{/each}
					</tr>
				{/each}
			</thead>
			<tbody>
				{#each $table_data.getRowModel().rows as row}
					<tr>
						{#each row.getVisibleCells() as cell}
							<td>
								<svelte:component
									this={flexRender(cell.column.columnDef.cell, cell.getContext())}
								/>
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
			<tfoot>
				{#each $table_data.getFooterGroups() as footerGroup}
					<tr>
						{#each footerGroup.headers as header}
							<th>
								{#if !header.isPlaceholder}
									<svelte:component
										this={flexRender(header.column.columnDef.footer, header.getContext())}
									/>
								{/if}
							</th>
						{/each}
					</tr>
				{/each}
			</tfoot>
		</table>
	</div>
{/await}
