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

// import { browser } from '$app/environment';

export class LocalStore<T> {
	value = $state<T>() as T;
	private key: string;
	private json: typeof JSON;

	constructor(key: string, value: T, json = JSON) {
		this.value = value;
		this.key = key;
		this.json = json;

		const item = localStorage.getItem(key);
		if (item) this.value = this.deserialize(item);

		$effect(() => {
			localStorage.setItem(this.key, this.serialize(this.value));
		});
	}

	private serialize(value: T): string {
		return this.json.stringify(value);
	}

	private deserialize(item: string): T {
		return this.json.parse(item);
	}
}

export function localStore<T>(key: string, value: T, json = JSON) {
	return new LocalStore(key, value, json);
}
