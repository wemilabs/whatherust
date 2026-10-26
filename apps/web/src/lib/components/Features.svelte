<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { formatMB, measurements } from '#lib/comparison.js';

	const icons = {
		webview: 'M3 5h18v14H3zM3 9h18M7 7h.01M10 7h.01',
		size: 'M12 3v12m0 0-4-4m4 4 4-4M5 21h14',
		tray: 'M4 17h16M6 13h12a2 2 0 0 0 2-2V5H4v6a2 2 0 0 0 2 2zM9 21h6',
		notifications: 'M18 8a6 6 0 1 0-12 0c0 7-3 9-3 9h18s-3-2-3-9M13.7 21a2 2 0 0 1-3.4 0',
		login:
			'M12 15v2m-6 4h12a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2zm10-10V7a4 4 0 0 0-8 0v4',
		single: 'M4 4h16v16H4zM9 9h6v6H9z',
		links: 'M14 4h6v6M10 14 20 4M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5',
		look: 'M12 3a9 9 0 1 0 0 18c1 0 1.5-.8 1.5-1.5 0-.4-.2-.8-.4-1.1-.3-.3-.4-.7-.4-1.1 0-.8.7-1.5 1.5-1.5H16a5 5 0 0 0 5-5c0-4.4-4-7.8-9-7.8zM7.5 12h.01M10 8h.01M14 8h.01',
		updates: 'M21 12a9 9 0 1 1-3-6.7L21 8M21 3v5h-5'
	};

	const features = $derived([
		{ icon: icons.webview, title: m.featureWebviewTitle(), body: m.featureWebviewBody() },
		{
			icon: icons.size,
			title: m.featureSizeTitle(),
			body: m.featureSizeBody({ size: formatMB(measurements.whatherust.deb) })
		},
		{ icon: icons.tray, title: m.featureTrayTitle(), body: m.featureTrayBody() },
		{
			icon: icons.notifications,
			title: m.featureNotificationsTitle(),
			body: m.featureNotificationsBody()
		},
		{ icon: icons.login, title: m.featureLoginTitle(), body: m.featureLoginBody() },
		{ icon: icons.single, title: m.featureSingleTitle(), body: m.featureSingleBody() },
		{ icon: icons.links, title: m.featureLinksTitle(), body: m.featureLinksBody() },
		{ icon: icons.look, title: m.featureLookTitle(), body: m.featureLookBody() },
		{ icon: icons.updates, title: m.featureUpdatesTitle(), body: m.featureUpdatesBody() }
	]);
</script>

<section
	id="features"
	aria-labelledby="features-title"
	class="mx-auto max-w-6xl px-4 py-16 sm:px-6 sm:py-20"
>
	<h2 id="features-title" class="text-3xl font-bold tracking-tight sm:text-4xl">
		{m.featuresTitle()}
	</h2>
	<p class="mt-3 text-stone-600 dark:text-stone-400">{m.featuresIntro()}</p>
	<ul class="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
		{#each features as feature (feature.title)}
			<li
				class="rounded-2xl border border-stone-200 bg-white p-6 dark:border-stone-800 dark:bg-stone-900"
			>
				<span
					class="inline-flex size-10 items-center justify-center rounded-xl bg-rust-50 text-rust-700 dark:bg-rust-400/15 dark:text-rust-400"
				>
					<svg
						class="size-5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						aria-hidden="true"
					>
						<path d={feature.icon} />
					</svg>
				</span>
				<h3 class="mt-4 font-semibold">{feature.title}</h3>
				<p class="mt-1.5 text-sm leading-relaxed text-stone-600 dark:text-stone-400">
					{feature.body}
				</p>
			</li>
		{/each}
	</ul>
</section>
