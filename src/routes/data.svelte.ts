// Copyright 2024 robonau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import type { Game } from '$lib';
import { localStore } from 'svelte-ux';

export const State = $state({
	dialogData: undefined as Game | undefined,
	coverOrBanner: 'cover' as 'cover' | 'banner',
	SGDBGame: undefined as undefined | SGDB_SearchResultData
});

export const API_TOKEN = localStore('API_TOKEN', '');

export type SGDB_Success<dataType, otherData = {}> =
	| ({
			success: true;
			data: dataType;
	  } & otherData)
	| {
			success: false;
	  };

export type SGDB_SearchResultData = {
	id: number;
	name: string;
	types: string[];
	verified: boolean;
};

export type SGDB_SearchResult = SGDB_Success<SGDB_SearchResultData[]>;

export type SGDB_ImageData = {
	id: number;
	score: number;
	style: string;
	url: string;
	thumb: string;
	tags: string[];
	height: number;
	width: number;
	upvotes: number;
	downvotes: number;
	language: string;
	author: {
		name: string;
		steam64: string;
		avatar: string;
	};
};

export type SGDB_ImageResult = SGDB_Success<
	SGDB_ImageData[],
	{ page: number; total: number; limit: number }
>;

export const SteamOriginalsCache = new Map<number, SGDB_ImageResult>();
export const cachedHeroes = new Map<number, SGDB_ImageResult>();
export const cachedGrids = new Map<number, SGDB_ImageResult>();
