<!--
 Copyright 2024 robonau
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     https://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { mdiChevronDown, mdiChevronUp, mdiHeart, mdiLoading } from '@mdi/js';
	import { invoke } from '@tauri-apps/api/core';
	import { fetch } from '@tauri-apps/plugin-http';
	import { Button, Icon } from 'svelte-ux';
	import { cubicInOut } from 'svelte/easing';
	import { slide } from 'svelte/transition';
	import type { SGDB_ImageData, SGDB_ImageResult } from '../data.svelte';
	import { API_TOKEN, cachedGrids, State, SteamOriginalsCache } from '../data.svelte';

	async function get_game_SteamOriginals(id: number): Promise<SGDB_ImageResult> {
		if (SteamOriginalsCache.has(id)) return SteamOriginalsCache.get(id)!;
		const steamresp = await fetch(
			`https://www.steamgriddb.com/api/v2/games/id/${id}?platformdata=steam`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${$API_TOKEN}`
				}
			}
		);
		const steamData = await steamresp.json();
		if (!steamData.success) return steamData;
		try {
			const steamID = steamData.data.external_platform_data.steam[0].id;
			const data = {
				success: true,
				data: [
					{
						id: 1,
						score: 0,
						style: '',
						url: `https://cdn.cloudflare.steamstatic.com/steam/apps/${steamID}/library_600x900_2x.jpg?t=${Date.now()}`,
						thumb: `https://cdn.cloudflare.steamstatic.com/steam/apps/${steamID}/library_600x900_2x.jpg?t=${Date.now()}`,
						tags: [''],
						height: 900,
						width: 600,
						upvotes: 0,
						downvotes: 0,
						language: '',
						author: {
							name: '',
							steam64: '',
							avatar: ''
						}
					},
					{
						id: 0,
						score: 0,
						style: '',
						url: `https://cdn.cloudflare.steamstatic.com/steam/apps/${steamID}/header.jpg?t=${Date.now()}`,
						thumb: `https://cdn.cloudflare.steamstatic.com/steam/apps/${steamID}/header.jpg?t=${Date.now()}`,
						tags: [''],
						height: 215,
						width: 460,
						upvotes: 0,
						downvotes: 0,
						language: '',
						author: {
							name: '',
							steam64: '',
							avatar: ''
						}
					}
				],
				total: 1,
				page: 1,
				limit: 1
			};
			SteamOriginalsCache.set(id, data);
			return data;
		} catch (error) {
			return {
				success: false
			};
		}
	}

	async function get_game_grids_SGDB(id: number): Promise<SGDB_ImageResult> {
		if (cachedGrids.has(id)) return cachedGrids.get(id)!;
		const resp = await fetch(
			`https://www.steamgriddb.com/api/v2/grids/game/${id}?nsfw=any&humor=any`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${$API_TOKEN}`
				}
			}
		);
		const data = await resp.json();
		cachedGrids.set(id, data);
		return data;
	}

	async function get_game_heros_SGDB(id: number): Promise<SGDB_ImageResult> {
		if (cachedGrids.has(id)) return cachedGrids.get(id)!;
		const resp = await fetch(
			'https://www.steamgriddb.com/api/v2/heroes/game/' + id + '?nsfw=any&humor=any',
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${$API_TOKEN}`
				}
			}
		);
		const data = await resp.json();
		cachedGrids.set(id, data);
		return data;
	}

	async function get_images(id: number) {
		const promises = [
			get_game_grids_SGDB(id),
			get_game_heros_SGDB(id),
			get_game_SteamOriginals(id)
		];
		const images = await Promise.all(promises);
		return images;
	}

	function sortData(data: SGDB_ImageData[]) {
		// return data.toSorted((a, b) => a.score - b.score);
		return data;
	}

	function Arr2NthArrs<T extends { height: number; width: number }>(arr: T[], n: number): T[][] {
		console.log(arr);
		return arr
			.reduce(
				(acc, c, i) => {
					if (!acc[i % n]) {
						acc.push({ data: [c], aspect: c.height / c.width });
						return acc;
					}
					let smallest = 0;
					let counter = Infinity;
					acc.forEach((_, i) => {
						if (acc[i].aspect < counter) {
							smallest = i;
							counter = acc[i].aspect;
						}
					});
					acc[smallest].data.push(c);
					acc[smallest].aspect += c.height / c.width;
					return acc;
				},
				[] as { data: T[]; aspect: number }[]
			)
			.map((x) => x.data);
	}

	if (!State.dialogData) {
		goto('/');
	}

	let showGrids = $state(true);
	let showHeroes = $state(true);
</script>

<div class="sticky top-0 z-[70] flex h-16 items-center justify-between bg-surface-200 px-4">
	<h1 class="text-2xl capitalize">
		{State.dialogData?.game.name}
		{State.coverOrBanner === 'cover' ? 'Cover' : 'Banner'}
	</h1>
	<div>
		<Button variant="fill" color="primary" on:click={() => goto('/')}>Back</Button>
	</div>
</div>

{#snippet btn(game: Extract<SGDB_ImageResult, { success: true }>['data'][number])}
	<button
		onclick={() => {
			if (!State.dialogData) return;
			invoke('replace_art', {
				path:
					State.coverOrBanner === 'cover'
						? State.dialogData.coverart_path
						: State.dialogData.banner_path,
				url: game.url
			}).then(() => {
				goto('/');
			});
		}}
		class="relative flex h-fit justify-center p-2 align-middle hover:bg-surface-200"
	>
		<img class="h-auto max-h-full w-full rounded-lg object-cover" src={game.thumb} alt="" />
		<div
			class="absolute bottom-2 left-2 right-2 flex justify-between rounded-b-olg bg-surface-300/40 backdrop-blur-lg backdrop-filter"
		>
			<div class="line-clamp-1 h-6 px-2 text-center" title={game.height + 'x' + game.width}>
				{game.height + 'x' + game.width}
			</div>
			<div
				class="line-clamp-1 flex h-6 items-center px-2 text-center"
				title={game.upvotes.toString()}
			>
				<Icon data={mdiChevronUp}></Icon>{game.upvotes}
				<Icon data={mdiChevronDown}></Icon>{game.downvotes}
			</div>
			<div
				class="line-clamp-1 flex h-6 items-center px-2 text-center"
				title={game.score.toString()}
			>
				<Icon data={mdiHeart}></Icon>{game.score}
			</div>
		</div>
	</button>
{/snippet}

<div class="h-full w-full">
	{#if State.dialogData && State.SGDBGame}
		{#await get_images(State.SGDBGame.id)}
			Loading data from SGDB<Icon data={mdiLoading} class="animate-spin" />
		{:then arr}
			{@const grid = arr[0]}
			{@const steam = arr[2]}
			{@const hero = arr[1]}

			<button
				onclick={() => (showGrids = !showGrids)}
				class="w-full p-2 text-2xl hover:bg-surface-200/50"
			>
				Grids <svg
					width="1.5em"
					height="1.5em"
					viewBox="0 0 24 24"
					class="inline-block fill-current transition-transform duration-200 ease-in-out
					{showGrids && 'rotate-180'}"
				>
					<path
						fill="currentColor"
						d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"
						class=" s-Yhd3BL-f1iBY"
					>
					</path>
				</svg>
			</button>
			{#if showGrids}
				<div transition:slide={{ axis: 'y', easing: cubicInOut }} class="w-full">
					{#if grid.success}
						{@const steamNGrid = steam.success ? [...steam.data, ...grid.data] : grid.data}
						<div class="grid grid-cols-6">
							{#each Arr2NthArrs(sortData(steamNGrid), 6) as masonry}
								<div class="grid h-fit">
									{#each masonry as game}
										{@render btn(game)}
									{/each}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
			<button
				onclick={() => (showHeroes = !showHeroes)}
				class="w-full p-2 text-2xl hover:bg-surface-200/50"
			>
				Heroes <svg
					width="1.5em"
					height="1.5em"
					viewBox="0 0 24 24"
					class="inline-block fill-current transition-transform duration-200 ease-in-out
					{showHeroes && 'rotate-180'}"
				>
					<path
						fill="currentColor"
						d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"
						class=" s-Yhd3BL-f1iBY"
					>
					</path>
				</svg>
			</button>
			{#if showHeroes}
				<div transition:slide={{ axis: 'y', easing: cubicInOut }} class="w-full">
					{#if hero.success}
						<div class="grid grid-cols-2 gap-4 md:grid-cols-4">
							{#each Arr2NthArrs(sortData(hero.data), 4) as masonry}
								<div class="grid h-fit gap-4">
									{#each masonry as game}
										{@render btn(game)}
									{/each}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		{:catch error}
			{error}
		{/await}
	{/if}
</div>
