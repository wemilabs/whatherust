<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { getLocale } from '#lib/paraglide/runtime.js';
	import { RELEASES_URL } from '#lib/site.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const dateFormat = $derived(
		new Intl.DateTimeFormat(getLocale(), { dateStyle: 'long', timeZone: 'UTC' })
	);
	const title = $derived(`${m.changelogTitle()} · WhatheRust`);
</script>

<svelte:head>
	<title>{title}</title>
	<meta property="og:title" content={title} />
	<meta name="twitter:title" content={title} />
</svelte:head>

<div class="mx-auto max-w-3xl px-4 py-14 sm:px-6 sm:py-20">
	<h1 class="text-4xl font-extrabold tracking-tight">{m.changelogTitle()}</h1>
	<p class="mt-3 text-stone-600 dark:text-stone-400">{m.changelogIntro()}</p>

	{#if data.releases.length === 0}
		<div
			class="mt-10 rounded-2xl border border-dashed border-stone-300 p-8 text-center dark:border-stone-700"
		>
			<p class="text-stone-700 dark:text-stone-300">{m.changelogEmpty()}</p>
			<a
				href={RELEASES_URL}
				rel="external"
				class="mt-4 inline-block font-semibold text-wa-800 underline underline-offset-4 dark:text-wa-300"
				>{m.changelogAll()}</a
			>
		</div>
	{:else}
		<ol class="mt-10 space-y-6">
			{#each data.releases as release (release.tag)}
				<li
					class="rounded-2xl border border-stone-200 bg-white p-6 dark:border-stone-800 dark:bg-stone-900"
				>
					<article aria-labelledby="release-{release.tag}">
						<header class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
							<h2 id="release-{release.tag}" class="text-xl font-bold">
								{release.name}
								{#if release.prerelease}
									<span
										class="ml-2 rounded-full bg-rust-50 px-2 py-0.5 align-middle text-xs font-semibold text-rust-700 dark:bg-rust-400/15 dark:text-rust-400"
										>{m.prerelease()}</span
									>
								{/if}
							</h2>
							{#if release.publishedAt}
								<time
									datetime={release.publishedAt}
									class="text-sm text-stone-600 dark:text-stone-400"
									>{dateFormat.format(new Date(release.publishedAt))}</time
								>
							{/if}
						</header>
						<div
							class="prose-release mt-2 text-sm leading-relaxed text-stone-700 dark:text-stone-300"
						>
							{@html release.html}
						</div>
						<a
							href={release.url}
							rel="external"
							class="mt-4 inline-block text-sm font-semibold text-wa-800 underline underline-offset-4 dark:text-wa-300"
							>{m.changelogOnGitHub()}</a
						>
					</article>
				</li>
			{/each}
		</ol>
		<a
			href={RELEASES_URL}
			rel="external"
			class="mt-8 inline-block text-sm font-semibold text-wa-800 underline underline-offset-4 dark:text-wa-300"
			>{m.changelogAll()}</a
		>
	{/if}
</div>
