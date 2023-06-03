<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar, AppRail, AppRailAnchor, LightSwitch } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import Home from '~icons/heroicons/home';
	import RectangleStack from '~icons/heroicons/rectangle-stack';
	import ClipboardDocumentList from '~icons/heroicons/clipboard-document-list';
	import UserCircle from '~icons/heroicons/user-circle';
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Off The Rail</strong>
				<p class="text-sm ml-2">The open source horse show management app</p>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch rounded="rounded-full" />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<AppRail>
			<AppRailAnchor href="/" selected={$page.url.pathname === '/'}>
				<svelte:fragment slot="lead">
					<Home class="aspect-square flex flex-col justify-center w-full text-2xl" />
				</svelte:fragment>
				<span>Main</span>
			</AppRailAnchor>
			<AppRailAnchor href="/shows" selected={$page.url.pathname.startsWith('/shows')}>
				<svelte:fragment slot="lead">
					<RectangleStack class="aspect-square flex flex-col justify-center w-full text-2xl" />
				</svelte:fragment>
				<span>Shows</span>
			</AppRailAnchor>
			<AppRailAnchor href="/score-entry" selected={$page.url.pathname === '/score-entry'}>
				<svelte:fragment slot="lead">
					<ClipboardDocumentList
						class="aspect-square flex flex-col justify-center w-full text-2xl"
					/>
				</svelte:fragment>
				<p>Score</p>
				<span>Entry</span>
			</AppRailAnchor>
			<AppRailAnchor href="/riders" selected={$page.url.pathname === '/riders'}>
				<svelte:fragment slot="lead">
					<UserCircle class="aspect-square flex flex-col justify-center w-full text-2xl" />
				</svelte:fragment>
				<span>Riders</span>
			</AppRailAnchor>
		</AppRail>
	</svelte:fragment>
	<!-- Page Route Content -->
	<div class="m-4">
		<slot />
	</div>
</AppShell>
