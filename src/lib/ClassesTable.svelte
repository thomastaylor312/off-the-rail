<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { createSvelteTable, getCoreRowModel, type Table } from '@tanstack/svelte-table';
	import type { ColumnDef, TableOptions } from '@tanstack/table-core/src/types';
	import type { Readable } from 'svelte/store';
	import { Class } from '$lib/types';
	import SortableTable from './SortableTable.svelte';

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
	<SortableTable {table_data} />
{/await}
