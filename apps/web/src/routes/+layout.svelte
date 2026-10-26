<script lang="ts">
	import { page } from '$app/state';
	import { m } from '#lib/paraglide/messages.js';
	import { baseLocale, getLocale, locales, localizeHref } from '#lib/paraglide/runtime.js';
	import { SITE_URL } from '#lib/site.js';
	import Header from '#lib/components/Header.svelte';
	import Footer from '#lib/components/Footer.svelte';
	import '@fontsource-variable/geist';
	import '@fontsource-variable/geist-mono';
	import './layout.css';
	import type { LayoutProps } from './$types';

	let { children, data }: LayoutProps = $props();

	const locale = $derived(getLocale());
	const absolute = (path: string, target: string) =>
		new URL(localizeHref(path, { locale: target as typeof baseLocale }), SITE_URL).href;
	const ogImage = $derived(`${SITE_URL}/og/${locale}.png`);
</script>

<svelte:head>
	<meta name="description" content={m.metaDescription()} />
	<link rel="canonical" href={absolute(page.url.pathname, locale)} />
	{#each locales as target (target)}
		<link rel="alternate" hreflang={target} href={absolute(page.url.pathname, target)} />
	{/each}
	<link rel="alternate" hreflang="x-default" href={absolute(page.url.pathname, baseLocale)} />
	<link rel="icon" href="/favicon.svg" type="image/svg+xml" />
	<link rel="icon" href="/favicon-32.png" sizes="32x32" type="image/png" />
	<link rel="apple-touch-icon" href="/apple-touch-icon.png" />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="WhatheRust" />
	<meta property="og:locale" content={locale === 'fr' ? 'fr_FR' : 'en_US'} />
	<meta property="og:url" content={absolute(page.url.pathname, locale)} />
	<meta property="og:description" content={m.metaDescription()} />
	<meta property="og:image" content={ogImage} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content={m.metaTitle()} />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:description" content={m.metaDescription()} />
	<meta name="twitter:image" content={ogImage} />
</svelte:head>

<a
	href="#main"
	class="sr-only z-30 rounded-lg bg-stone-900 px-4 py-2 text-white focus:not-sr-only focus:fixed focus:top-3 focus:left-3 dark:bg-white dark:text-stone-900"
>
	{m.skipToContent()}
</a>
<Header stars={data.stars} />
<main id="main">
	{@render children()}
</main>
<Footer />
