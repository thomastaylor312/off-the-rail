<script lang="ts">
	import { flexRender } from '$lib/types';
	import type { Readable } from 'svelte/store';

	export let table_data: Readable<import('@tanstack/table-core').Table<any>>;
</script>

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
							<svelte:component this={flexRender(cell.column.columnDef.cell, cell.getContext())} />
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
