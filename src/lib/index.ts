// place files you want to import through the `$lib` alias in this folder.

import { invoke } from '@tauri-apps/api/core';

type SubGame = {
	id: number;
	name: string;
	slug: string;
	lastplayed?: number;
	installed_at?: number;
	has_custom_banner: number;
	has_custom_coverart_big: number;
	playtime: number;
};

export type Game = {
	game: SubGame;
	coverart_path: string;
	coverart_width: number;
	coverart_height: number;
	banner_path: string;
	banner_width: number;
	banner_height: number;
};

export function isGames(obj: Game[] | string): obj is Game[] {
	return Array.isArray(obj);
}

export async function get_games(): Promise<Game[] | string> {
	return (await invoke('get_games')) as Game[] | string;
}
export type CssClasses = string;

export const gridValues =
	'xs:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-7 2xl:grid-cols-8 3xl:grid-cols-10';

export const screens = {
	xs: '320px',
	sm: '640px',
	md: '768px',
	lg: '1024px',
	xl: '1280px',
	'2xl': '1536px',
	'3xl': '2000px'
};
