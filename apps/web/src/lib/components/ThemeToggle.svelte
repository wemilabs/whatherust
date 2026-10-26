<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '#lib/paraglide/messages.js';

	let dark = $state(false);

	function apply(value: boolean) {
		dark = value;
		document.documentElement.classList.toggle('dark', value);
		document.documentElement.style.colorScheme = value ? 'dark' : 'light';
	}

	onMount(() => {
		dark = document.documentElement.classList.contains('dark');
		const media = matchMedia('(prefers-color-scheme: dark)');
		const onChange = (event: MediaQueryListEvent) => {
			let stored: string | null = null;
			try {
				stored = localStorage.getItem('theme');
			} catch {
				// Storage can be unavailable in private modes.
			}
			if (!stored) apply(event.matches);
		};
		media.addEventListener('change', onChange);
		return () => media.removeEventListener('change', onChange);
	});

	function toggle() {
		apply(!dark);
		try {
			localStorage.setItem('theme', dark ? 'dark' : 'light');
		} catch {
			// The choice still applies for this page view.
		}
	}
</script>

<button
	type="button"
	onclick={toggle}
	class="inline-flex size-10 items-center justify-center rounded-lg text-stone-700 transition-colors hover:bg-stone-200/70 dark:text-stone-300 dark:hover:bg-stone-800"
	aria-label={dark ? m.themeToLight() : m.themeToDark()}
	title={dark ? m.themeToLight() : m.themeToDark()}
>
	<svg
		class="size-5 dark:hidden"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		aria-hidden="true"
	>
		<path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" />
	</svg>
	<svg
		class="hidden size-5 dark:block"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		aria-hidden="true"
	>
		<circle cx="12" cy="12" r="4" />
		<path
			d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"
		/>
	</svg>
</button>
