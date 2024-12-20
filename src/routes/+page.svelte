<script lang="ts">
	import { goto } from '$app/navigation';
	import { get_games, isGames, localStore, TriState, type Game } from '$lib';
	import ImageCard from '$lib/ImageCard.svelte';
	import { mdiApi, mdiFilter, mdiLoading, mdiSort } from '@mdi/js';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { fetch } from '@tauri-apps/plugin-http';
	import { Button, Checkbox, Dialog, Drawer, Icon, SelectField, TextField } from 'svelte-ux';
	import { API_TOKEN, State, type SGDB_SearchResult } from './data.svelte';
	import ThreeStateSwitch from '$lib/ThreeStateSwitch.svelte';
	let games: Game[] = $state([]);
	get_games().then((ret) => {
		if (!isGames(ret)) return;
		console.log(ret);
		games = ret;
	});
	let drawer = $state('');
	const filterBy = localStore('filterBy', {
		search: '',
		has_custom_banner: TriState.Ignore,
		has_custom_coverart_big: TriState.Ignore
	});

	let open = $state(false);
	let open2 = $state(false);

	const sortBy = localStore('sortBy', {
		asc: false,
		by: 'installed_at' as 'installed_at' | 'lastplayed' | 'playtime'
	});

	let filteredGames = $derived.by(() => {
		return games.filter((game) => {
			let tru = true;
			if (filterBy.value.search) {
				tru = game.game.name.toLowerCase().includes(filterBy.value.search.toLowerCase());
			}
			if (filterBy.value.has_custom_banner !== TriState.Ignore) {
				tru =
					tru &&
					(game.game.has_custom_banner === 1) ===
						(filterBy.value.has_custom_banner === TriState.Include);
			}
			if (filterBy.value.has_custom_coverart_big !== TriState.Ignore) {
				tru =
					tru &&
					(game.game.has_custom_coverart_big === 1) ===
						(filterBy.value.has_custom_coverart_big === TriState.Include);
			}

			return tru;
		});
	});

	let sortedGames = $derived.by(() => {
		return filteredGames.toSorted((a, b) => {
			if (sortBy.value.asc)
				return (a.game[sortBy.value.by] ?? Infinity) - (b.game[sortBy.value.by] ?? Infinity);
			return (b.game[sortBy.value.by] ?? Infinity) - (a.game[sortBy.value.by] ?? Infinity);
		});
	});

	let cachedSearch = undefined as { name: string; data: SGDB_SearchResult } | undefined;

	async function get_games_SGDB(name: string): Promise<SGDB_SearchResult> {
		if (cachedSearch?.name === name) return cachedSearch.data;
		const resp = await fetch('https://www.steamgriddb.com/api/v2/search/autocomplete/' + name, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${$API_TOKEN}`
			}
		});
		const data = await resp.json();
		cachedSearch = { name, data };
		return data;
	}
	const date = Date.now();
</script>

<div class="sticky top-0 z-[70] flex h-16 items-center justify-between bg-surface-200 px-4">
	<h1 class="text-2xl">Lutris Games</h1>
	<div>
		<Drawer open={drawer === 'auth'} placement="right" class="w-[400px] pt-16">
			<div class="p-4">
				<h2 class="text-xl">Auth</h2>
				<TextField
					label="API token"
					on:change={(e) => {
						console.log(e.detail.value);
						API_TOKEN.set(e.detail.value as unknown as string);
					}}
				/>
			</div>
			<div slot="actions">
				<Button onclick={() => (drawer = drawer === 'auth' ? '' : 'auth')}>Close</Button>
			</div>
		</Drawer>
		<Button
			onclick={() => (drawer = drawer === 'auth' ? '' : 'auth')}
			class="rounded-full p-2 hover:bg-surface-100"
		>
			<Icon data={mdiApi} />
		</Button>
		<Drawer open={drawer === 'sort'} placement="right" class="w-[400px] pt-16">
			<div class="p-4">
				<h2 class="text-xl">Sorts</h2>
				<Checkbox bind:checked={sortBy.value.asc}>Asc</Checkbox>
				<SelectField
					options={[
						{ label: 'Installed at', value: 'installed_at' },
						{ label: 'Last played', value: 'lastplayed' },
						{ label: 'Playtime', value: 'playtime' }
					]}
					bind:value={sortBy.value.by}
				/>
			</div>
			<div slot="actions">
				<Button onclick={() => (drawer = drawer === 'sort' ? '' : 'sort')}>Close</Button>
			</div>
		</Drawer>
		<Button
			onclick={() => (drawer = drawer === 'sort' ? '' : 'sort')}
			class="rounded-full p-2 hover:bg-surface-100"
		>
			<Icon data={mdiSort} />
		</Button>
		<Drawer open={drawer === 'filter'} placement="right" class="w-[400px] pt-16">
			<div class="p-4">
				<h2 class="text-xl">Filters</h2>
				<TextField
					label="Search"
					on:change={(e) => {
						filterBy.value.search = e.detail.value as unknown as string;
					}}
				/>
				<label
					class="flex w-full select-none items-center gap-2 rounded-lg p-2 text-sm hover:bg-surface-200/50"
				>
					<ThreeStateSwitch bind:State={filterBy.value.has_custom_banner}></ThreeStateSwitch>
					has custom banner
				</label>
				<label
					class="flex w-full select-none items-center gap-2 rounded-lg p-2 text-sm hover:bg-surface-200/50"
				>
					<ThreeStateSwitch bind:State={filterBy.value.has_custom_coverart_big}></ThreeStateSwitch>
					has custom cover
				</label>
			</div>
			<div slot="actions">
				<Button onclick={() => (drawer = drawer === 'filter' ? '' : 'filter')}>Close</Button>
			</div>
		</Drawer>
		<Button
			onclick={() => (drawer = drawer === 'filter' ? '' : 'filter')}
			class="rounded-full p-2 hover:bg-surface-100"
		>
			<Icon data={mdiFilter} />
		</Button>
	</div>
</div>

<Dialog bind:open={open2} class="mt-16">
	<div slot="title">
		{State.dialogData?.game.name}
		{State.coverOrBanner === 'cover' ? 'Cover' : 'Banner'}
	</div>
	<div class="max-h-[calc(100vh-12rem)] overflow-auto">
		{#if State.dialogData}
			{#await get_games_SGDB(State.dialogData.game.name)}
				Loading data from SGDB<Icon data={mdiLoading} class="animate-spin" />
			{:then data}
				{#if data.success}
					{#each data.data as game}
						<button
							class="block w-full p-2 hover:bg-surface-200"
							onclick={() => {
								State.SGDBGame = game;
								goto('/images');
							}}
						>
							{game.name}
						</button>
					{/each}
				{/if}
			{/await}
		{/if}
	</div>
	<div slot="actions">
		<Button variant="fill" color="primary">Close</Button>
	</div>
</Dialog>

<Dialog bind:open class="mt-16">
	<div slot="title">{State.dialogData?.game.name}</div>
	{#if State.dialogData}
		<div class="flex h-[calc(100vh-12rem)] flex-col flex-nowrap items-center">
			<button
				onclick={() => {
					open2 = true;
					State.coverOrBanner = 'cover';
				}}
				class="h-[67%] flex-1 rounded-lg p-2 hover:bg-surface-200"
			>
				<img
					class="m-auto h-full w-auto rounded-lg object-contain"
					src={convertFileSrc(State.dialogData.coverart_path) + '?ts=' + date}
					alt={State.dialogData.game.name}
				/>
			</button>
			<button
				onclick={() => {
					open2 = true;
					State.coverOrBanner = 'banner';
				}}
				class="h-[33%] flex-1 rounded-lg p-2 hover:bg-surface-200"
			>
				<img
					class="m-auto h-full w-auto rounded-lg object-contain"
					src={convertFileSrc(State.dialogData.banner_path) + '?ts=' + date}
					alt={State.dialogData.game.name}
				/>
			</button>
		</div>
	{/if}
	<div slot="actions">
		<Button variant="fill" color="primary">Close</Button>
	</div>
</Dialog>

<div
	class="m-2 grid grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-7 2xl:grid-cols-8 3xl:grid-cols-10"
>
	{#each sortedGames as game}
		<button
			onclick={() => {
				State.dialogData = game;
				open = true;
			}}
		>
			<div class="aspect-cover">
				<ImageCard
					thumbnailUrl={convertFileSrc(game.coverart_path) + '?ts=' + date}
					title={game.game.name}
					draggable={false}
					class="select-none"
					rounded="rounded-lg"
				>
					<div
						class="absolute bottom-0 left-0 right-0 rounded-b-olg bg-surface-300/40 backdrop-blur-lg backdrop-filter"
					>
						<div
							class="h-6 w-full overflow-hidden overflow-ellipsis whitespace-nowrap px-2 text-center"
							title={game.game.name}
						>
							{game.game.name}
						</div>
						<div
							class="flex h-6 w-full justify-between overflow-hidden overflow-ellipsis whitespace-nowrap px-2 text-center"
						>
							<div>
								{game.coverart_width}x{game.coverart_height}
							</div>
							<div>
								{game.banner_width}x{game.banner_height}
							</div>
						</div>
					</div>
				</ImageCard>
			</div>
		</button>
	{/each}
</div>
