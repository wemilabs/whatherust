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

<section class="relative overflow-hidden">
	<div
		class="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(60rem_30rem_at_85%_-10%,--theme(--color-rust-400/18%),transparent),radial-gradient(50rem_30rem_at_0%_10%,--theme(--color-wa-400/14%),transparent)]"
		aria-hidden="true"
	></div>
	<div
		class="mx-auto grid max-w-6xl items-center gap-12 px-4 pt-14 pb-16 sm:px-6 sm:pt-20 lg:grid-cols-[1.35fr_1fr] lg:pb-24"
	>
		<div class="text-center sm:text-left">
			<p class="text-sm font-semibold tracking-wide text-rust-700 dark:text-rust-400">
				{m.heroEyebrow()}
			</p>
			<h1 class="mt-3 text-4xl font-extrabold tracking-tight text-balance sm:text-5xl lg:text-6xl">
				{m.heroTitle()}
			</h1>
			<p
				class="mt-5 max-w-2xl text-lg leading-relaxed text-pretty text-stone-700 dark:text-stone-300"
			>
				{m.heroSubtitle()}
			</p>
			<p class="mt-4 text-sm text-stone-600 dark:text-stone-400">
				<a
					href={REPO_URL}
					rel="external"
					class="font-semibold text-wa-800 underline underline-offset-4 dark:text-wa-300"
					>{m.openSource()}</a
				>
				· {m.licenseNone()}
			</p>
			<div class="mt-8">
				<Download latest={data.latest} />
			</div>
			<p class="mt-8 max-w-2xl text-xs leading-relaxed text-stone-500 dark:text-stone-400">
				{m.disclaimer()}
			</p>
		</div>
		<div class="hidden justify-center lg:flex" aria-hidden="true">
			<img src={logo} alt="" width="360" height="360" class="size-80 drop-shadow-2xl xl:size-90" />
		</div>
	</div>
</section>

<Features />
<Screenshots />
<Comparison />
