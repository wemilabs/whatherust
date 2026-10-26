<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { getLocale } from '#lib/paraglide/runtime.js';
	import { formatMB, formatMiB, measuredOn, measurements } from '#lib/comparison.js';

	const { whatherust, electron } = measurements;

	const groups = $derived([
		{
			id: 'size',
			title: m.compareSize(),
			metrics: [
				{
					label: m.compareSizeDeb(),
					rows: [
						{
							name: whatherust.name,
							value: whatherust.deb,
							text: formatMB(whatherust.deb),
							ours: true
						},
						{ name: electron.name, value: electron.deb, text: formatMB(electron.deb), ours: false }
					]
				},
				{
					label: m.compareSizeAppImage(),
					rows: [
						{
							name: whatherust.name,
							value: whatherust.appimage,
							text: formatMB(whatherust.appimage),
							ours: true
						},
						{
							name: electron.name,
							value: electron.appimage,
							text: formatMB(electron.appimage),
							ours: false
						}
					]
				}
			]
		},
		{
			id: 'memory',
			title: m.compareMemory(),
			metrics: [
				{
					label: 'RSS',
					rows: [
						{
							name: whatherust.name,
							value: whatherust.rssMiB,
							text: formatMiB(whatherust.rssMiB),
							ours: true
						},
						{
							name: electron.name,
							value: electron.rssMiB,
							text: formatMiB(electron.rssMiB),
							ours: false
						}
					]
				}
			]
		}
	]);

	const date = $derived(
		new Intl.DateTimeFormat(getLocale(), { dateStyle: 'long', timeZone: 'UTC' }).format(
			new Date(measuredOn)
		)
	);

	const sources = [
		{
			label: 'Windows Latest, 21 Jul 2025',
			href: 'https://www.windowslatest.com/2025/07/21/whatsapp-for-windows-11-is-switching-back-to-chromium-web-wrapper-from-uwp-native/'
		},
		{
			label: 'Daring Fireball, 15 Nov 2025',
			href: 'https://daringfireball.net/2025/11/meta_whatsapp_windows_shitty_web_app'
		}
	];
</script>

<section
	id="comparison"
	aria-labelledby="comparison-title"
	class="mx-auto max-w-6xl px-4 py-16 sm:px-6 sm:py-20"
>
	<h2 id="comparison-title" class="text-3xl font-bold tracking-tight sm:text-4xl">
		{m.compareTitle()}
	</h2>
	<p class="mt-3 max-w-3xl text-stone-600 dark:text-stone-400">{m.compareIntro()}</p>

	<div class="mt-10 grid gap-6 lg:grid-cols-2">
		{#each groups as group (group.id)}
			<figure
				class="rounded-2xl border border-stone-200 bg-white p-6 dark:border-stone-800 dark:bg-stone-900"
				aria-labelledby="{group.id}-title"
			>
				<figcaption>
					<h3 id="{group.id}-title" class="font-semibold">{group.title}</h3>
					<p class="text-sm text-stone-600 dark:text-stone-400">{m.compareLower()}</p>
				</figcaption>
				{#each group.metrics as metric (metric.label)}
					{@const max = Math.max(...metric.rows.map((row) => row.value))}
					<div class="mt-5">
						<p class="text-sm font-medium text-stone-700 dark:text-stone-300">{metric.label}</p>
						<dl class="mt-2 space-y-2">
							{#each metric.rows as row (row.name)}
								<div
									class="grid grid-cols-[7.5rem_1fr] items-center gap-3 text-sm sm:grid-cols-[9rem_1fr]"
								>
									<dt class="truncate text-stone-700 dark:text-stone-300">{row.name}</dt>
									<dd class="flex items-center gap-2">
										<span class="block min-w-0 flex-1"
											><svg
												class="block h-3 w-full"
												viewBox="0 0 100 4"
												preserveAspectRatio="none"
												aria-hidden="true"
											>
												<rect
													width={(row.value / max) * 100}
													height="4"
													rx="0.6"
													class={row.ours
														? 'fill-wa-700 dark:fill-wa-400'
														: 'fill-stone-400 dark:fill-stone-500'}
												/>
											</svg></span
										>
										<span class="w-20 shrink-0 text-right font-mono font-semibold tabular-nums"
											>{row.text}</span
										>
									</dd>
								</div>
							{/each}
						</dl>
					</div>
				{/each}
				{#if group.id === 'memory'}
					<p class="mt-5 text-sm leading-relaxed text-stone-600 dark:text-stone-400">
						{m.compareMemoryNote()}
					</p>
				{/if}
			</figure>
		{/each}
	</div>

	<p class="mt-8 max-w-3xl text-sm leading-relaxed text-stone-700 dark:text-stone-300">
		{m.compareOfficial()}
		{m.compareSources()}
		{#each sources as source, index (source.href)}
			<a
				href={source.href}
				rel="external"
				class="font-medium text-wa-800 underline underline-offset-4 dark:text-wa-300"
				>{source.label}</a
			>{index < sources.length - 1 ? ', ' : '.'}
		{/each}
	</p>

	<div
		class="mt-6 max-w-3xl rounded-xl bg-stone-100 p-4 text-xs leading-relaxed text-stone-600 dark:bg-stone-900 dark:text-stone-400"
	>
		<p class="font-semibold text-stone-700 dark:text-stone-300">{m.compareMethodTitle()}</p>
		<p class="mt-1">{m.compareMethod({ date })}</p>
	</div>
</section>
