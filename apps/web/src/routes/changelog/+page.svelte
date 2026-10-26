<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { getLocale } from '#lib/paraglide/runtime.js';
	import { RELEASES_URL } from '#lib/site.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const dateFormat = $derived(
		new Intl.DateTimeFormat(getLocale(), { dateStyle: 'long', timeZone: 'UTC' })
	);
	const title = $derived(`${m.changelogLead()} ${m.changelogTitle()} · WhatheRust`);
</script>

<svelte:head>
	<title>{title}</title>
	<meta property="og:title" content={title} />
	<meta name="twitter:title" content={title} />
</svelte:head>

<div class="px-4 py-16 sm:px-8 sm:py-20">
	<p class="font-mono text-xs text-muted tabular-nums">04</p>
	<h1 class="mt-5 text-[40px] sm:text-[56px]">
		<span class="text-subtle">{m.changelogLead()}</span>
		<span class="block">{m.changelogTitle()}</span>
	</h1>
	<p class="mt-5 max-w-2xl text-base leading-relaxed text-muted sm:text-lg">{m.changelogIntro()}</p>

	{#if data.releases.length === 0}
		<div class="mt-10 rounded-xl border border-dashed border-line-strong p-8 text-center">
			<p class="text-muted">{m.changelogEmpty()}</p>
			<a
				href={RELEASES_URL}
				rel="external"
				class="mt-4 inline-block text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
				>{m.changelogAll()}</a
			>
		</div>
	{:else}
		<ol class="mt-10">
			{#each data.releases as release (release.tag)}
				<li class="border-t border-line py-8">
					<div class="grid gap-4 sm:grid-cols-[7rem_minmax(0,1fr)] sm:gap-8">
						{#if release.publishedAt}
							<time
								datetime={release.publishedAt}
								class="font-mono text-xs text-muted tabular-nums sm:w-28"
								>{dateFormat.format(new Date(release.publishedAt))}</time
							>
						{:else}
							<span class="font-mono text-xs text-muted sm:w-28">—</span>
						{/if}
						<article aria-labelledby="release-{release.tag}">
							<header class="flex flex-wrap items-center gap-3">
								<h2 id="release-{release.tag}" class="text-xl">{release.name}</h2>
								{#if release.prerelease}
									<span
										class="rounded-full border border-line-strong px-2 py-0.5 text-xs text-muted"
										>{m.prerelease()}</span
									>
								{/if}
							</header>
							<div class="prose-release mt-4 max-w-3xl text-base leading-relaxed text-muted">
								{@html release.html}
							</div>
							<a
								href={release.url}
								rel="external"
								class="mt-4 inline-block text-sm text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
								>{m.changelogOnGitHub()}</a
							>
						</article>
					</div>
				</li>
			{/each}
		</ol>
		<a
			href={RELEASES_URL}
			rel="external"
			class="mt-8 inline-block text-sm text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
			>{m.changelogAll()}</a
		>
	{/if}
</div>
