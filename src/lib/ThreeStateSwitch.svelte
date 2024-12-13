<script lang="ts">
	import type { TriState } from '$lib';
	import { mdiCheck, mdiClose } from '@mdi/js';
	import { cls, getComponentClasses, Icon, uniqueId, type ThemeColors } from 'svelte-ux';

	interface Props {
		id?: string;
		name?: string;
		value?: any;
		State?: (typeof TriState)[keyof typeof TriState];
		required?: boolean;
		disabled?: boolean;
		size?: 'sm' | 'md' | 'lg';
		color?: ThemeColors;
		classes?: {
			root?: string;
			input?: string;
			switch?: string;
			toggle?: string;
		};
		onchange?: (event: Event) => void;
		children?: import('svelte').Snippet<[any]>;
		[key: string]: unknown;
	}

	let {
		id = uniqueId('switch-'),
		name = '',
		value = undefined,
		State = $bindable('Ignore'),
		required = false,
		disabled = false,
		size = 'lg',
		color = 'primary',
		classes = {},
		onchange = () => {},
		children,
		...rest
	}: Props = $props();
	const settingsClasses = getComponentClasses('Switch');
	function change() {
		switch (State) {
			case 'Exclude':
				State = 'Ignore';
				break;
			case 'Include':
				State = 'Exclude';
				break;
			case 'Ignore':
				State = 'Include';
				break;
			default:
				State = 'Ignore';
				break;
		}
	}
</script>

<div class={cls('Switch', 'inline-block', settingsClasses.root, classes.root)}>
	<input
		{id}
		{name}
		type="checkbox"
		checked={State === 'Include'}
		indeterminate={State === 'Ignore'}
		onchange={change}
		{value}
		class={cls('peer block h-0 appearance-none', settingsClasses.input, classes.input)}
		{required}
		{disabled}
	/>

	<label
		for={id}
		data-State={State}
		class={cls(
			'switch',
			'align-items group grid rounded-full border p-[2px] transition-shadow',
			{
				'h-4 w-6': size === 'sm',
				'h-5 w-8': size === 'md',
				'h-6 w-10': size === 'lg'
			},
			State === 'Include' &&
				{
					primary: 'border-primary bg-primary',
					secondary: 'border-secondary bg-secondary',
					accent: 'border-accent bg-accent',
					neutral: 'border-neutral bg-neutral',
					info: 'border-info bg-info',
					success: 'border-success bg-success',
					warning: 'border-warning bg-warning',
					danger: 'border-danger bg-danger'
				}[color],
			{
				primary: 'ring-primary/60',
				secondary: 'ring-secondary/60',
				accent: 'ring-accent/60',
				neutral: 'ring-neutral/60',
				info: 'ring-info/60',
				success: 'ring-success/60',
				warning: 'ring-warning/60',
				danger: 'ring-danger/60'
			}[color],
			State === 'Ignore' && 'bg-surface-content/20',
			State === 'Exclude' && 'bg-surface-content/60',
			disabled ? 'opacity-50' : 'cursor-pointer ring-offset-1 peer-focus-visible:ring-2',
			settingsClasses.switch,
			classes.switch,
			rest.class ?? ''
		)}
	>
		<div
			data-State={State}
			class={cls(
				'toggle grid aspect-square h-full w-1/2 transform items-center justify-center rounded-full bg-surface-100 transition-all duration-200',
				'aspect-auto group-active:w-[60%]',
				State === 'Include' && 'translate-x-full group-active:translate-x-[65%]',
				State === 'Ignore' && 'translate-x-1/2 border',
				settingsClasses.toggle,
				classes.toggle
			)}
		>
			{#if State === 'Include'}
				<Icon data={mdiCheck} class="text-primary" size=".8em" />
			{:else if State === 'Exclude'}
				<Icon data={mdiClose} class="text-surface-content" size=".8em" />
			{/if}
		</div>
	</label>
</div>
