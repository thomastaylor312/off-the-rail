<script lang="ts">
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import RidersTable from '$lib/RidersTable.svelte';

	let tabSet: number = 0;
</script>

<TabGroup>
	<Tab bind:group={tabSet} name="tab1" value={0}>Riders</Tab>
	<Tab bind:group={tabSet} name="tab2" value={1}>Horses</Tab>
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#if tabSet === 0}
			<RidersTable />
		{:else if tabSet === 1}
			(tab panel 2 contents)
		{/if}
	</svelte:fragment>
</TabGroup>
