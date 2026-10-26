<script lang="ts">
	import type { Path } from '$app/types';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { m } from '#lib/paraglide/messages.js';
	import { getLocale, localizeHref, locales } from '#lib/paraglide/runtime.js';
	import { REPO_URL } from '#lib/site.js';
	import logo from '#lib/assets/logo.svg';
	import ThemeToggle from './ThemeToggle.svelte';

	let { stars }: { stars: number | null } = $props();

	const locale = $derived(getLocale());
	const starsLabel = $derived(
		stars === null ? null : new Intl.NumberFormat(locale, { notation: 'compact' }).format(stars)
	);
	const href = (path: string, target = locale) =>
		resolve(localizeHref(path, { locale: target }) as Path);
</script>

<header
	class="sticky top-0 z-20 border-b border-stone-200/80 bg-stone-50/85 backdrop-blur dark:border-stone-800/80 dark:bg-stone-950/85"
>
	<div class="mx-auto flex h-16 max-w-6xl items-center justify-between gap-2 px-4 sm:px-6">
		<a
			href={href('/')}
			class="flex min-w-0 items-center gap-2 rounded-lg font-semibold tracking-tight"
		>
			<img src={logo} alt="" width="32" height="32" class="size-8 shrink-0" />
			<span class="text-lg">WhatheRust</span>
		</a>
		<nav class="flex items-center gap-1 sm:gap-2">
			<a
				href={href('/changelog')}
				class="hidden rounded-lg px-3 py-2 text-sm font-medium text-stone-700 hover:bg-stone-200/70 sm:inline-block dark:text-stone-300 dark:hover:bg-stone-800"
				aria-current={page.url.pathname.endsWith('/changelog') ? 'page' : undefined}
			>
				{m.navChangelog()}
			</a>
			<a
				href={REPO_URL}
				rel="external"
				class="inline-flex h-10 items-center gap-1.5 rounded-lg px-2.5 text-sm font-medium text-stone-700 hover:bg-stone-200/70 dark:text-stone-300 dark:hover:bg-stone-800"
				aria-label={starsLabel === null ? m.githubLink() : m.githubStars({ stars: String(stars) })}
			>
				<svg class="size-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
					<path
						d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2.2c-3.3.7-4-1.4-4-1.4-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.4 1.2a11.5 11.5 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.6.2 2.8.1 3.2.8.8 1.3 1.9 1.3 3.2 0 4.6-2.8 5.6-5.5 5.9.4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .3"
					/>
				</svg>
				{#if starsLabel !== null}
					<span class="whitespace-nowrap tabular-nums" aria-hidden="true">★ {starsLabel}</span>
				{/if}
			</a>
			<ThemeToggle />
			<div
				role="group"
				aria-label={m.languageLabel()}
				class="flex rounded-lg border border-stone-300 p-0.5 text-xs font-semibold dark:border-stone-700"
			>
				{#each locales as target (target)}
					<a
						href={href(page.url.pathname, target)}
						hreflang={target}
						lang={target}
						data-sveltekit-reload
						aria-current={target === locale ? 'true' : undefined}
						class="rounded-md px-2 py-1.5 uppercase {target === locale
							? 'bg-stone-900 text-white dark:bg-stone-100 dark:text-stone-900'
							: 'text-stone-600 hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-100'}"
					>
						{target}
					</a>
				{/each}
			</div>
		</nav>
	</div>
</header>
