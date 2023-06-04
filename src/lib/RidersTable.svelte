<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { createSvelteTable, getCoreRowModel, type Table } from '@tanstack/svelte-table';
	import type { ColumnDef, TableOptions } from '@tanstack/table-core/src/types';
	import type { Readable } from 'svelte/store';
	import { Rider } from '$lib/types';
	import SortableTable from './SortableTable.svelte';

	const columns: ColumnDef<Rider>[] = [
		{
			accessorKey: 'name',
			cell: (info) => info.getValue(),
			header: () => 'Name'
		},
		{
			accessorKey: 'email',
			cell: (info) => info.getValue(),
			header: () => 'Email'
		},
		{
			accessorKey: 'membership_date',
			// TODO: Make sure the membership date is less than 1 year old as well
			cell: (info) => {
				let val = info.getValue<Date>();
				return val ? val.toLocaleDateString() : 'Not a member';
			},
			header: () => 'Membership Date'
		},
		{
			accessorKey: 'phone',
			cell: (info) => info.getValue(),
			header: () => 'Phone'
		}
	];

	let table_data: Readable<Table<Rider>>;

	async function getClasses(): Promise<void> {
		let raw = await invoke<object[]>('list_riders').catch((err: string) => {
			// TODO: Actually send this to an error toast
			console.error(err);
		});

		let data = raw?.map((cls) => {
			return new Rider(cls);
		});

		console.log(data);

		let options: TableOptions<Rider> = {
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
