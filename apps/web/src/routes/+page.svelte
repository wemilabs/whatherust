<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { REPO_URL } from '#lib/site.js';
	import logo from '#lib/assets/logo.svg';
	import Download from '#lib/components/Download.svelte';
	import Features from '#lib/components/Features.svelte';
	import Screenshots from '#lib/components/Screenshots.svelte';
	import Comparison from '#lib/components/Comparison.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>{m.metaTitle()}</title>
	<meta property="og:title" content={m.metaTitle()} />
	<meta name="twitter:title" content={m.metaTitle()} />
</svelte:head>

<section aria-labelledby="hero-title">
	<div
		class="grid items-center gap-12 px-4 pt-16 pb-16 sm:px-8 sm:pt-20 sm:pb-20 lg:grid-cols-[1.35fr_1fr] lg:pt-24 lg:pb-24"
	>
		<div>
			<h1 id="hero-title" class="max-w-2xl text-[40px] text-balance sm:text-[56px] lg:text-[64px]">
				{m.heroTitle()}
			</h1>
			<p class="mt-6 max-w-xl text-lg leading-relaxed text-pretty text-muted">
				{m.heroSubtitle()}
			</p>
			<p class="mt-5 text-sm text-muted">
				<a
					href={REPO_URL}
					target="_blank"
					rel="external noopener noreferrer"
					class="text-rust-700 underline decoration-transparent underline-offset-4 transition hover:decoration-current dark:text-rust-400"
					>{m.openSource()}</a
				>
				<span aria-hidden="true"> · </span>
				<a
					href={`${REPO_URL}/blob/main/LICENSE`}
					target="_blank"
					rel="external noopener noreferrer"
					class="text-rust-700 underline decoration-transparent underline-offset-4 transition hover:decoration-current dark:text-rust-400"
					>{m.licenseMit()}</a
				>
			</p>
			<div class="mt-8">
				<Download latest={data.latest} />
			</div>
		</div>
		<div class="hidden justify-end lg:flex" aria-hidden="true">
			<div
				class="grid size-[340px] place-items-center rounded-2xl border border-line bg-surface"
				style="background-image: radial-gradient(var(--line) 1px, transparent 1px); background-size: 24px 24px;"
			>
				<img src={logo} alt="" width="280" height="280" class="size-[280px]" />
			</div>
		</div>
	</div>
</section>

<Features />
<Screenshots />
<Comparison />
