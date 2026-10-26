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

<section id="comparison" aria-labelledby="comparison-title" class="border-t border-line">
	<div class="px-4 py-16 sm:px-8 sm:py-20">
		<p class="font-mono text-xs text-muted tabular-nums">03</p>
		<h2 id="comparison-title" class="mt-5 text-[32px] sm:text-[40px]">
			<span class="text-subtle">{m.compareLead()}</span>
			<span class="block">{m.compareTitle()}</span>
		</h2>
		<p class="mt-5 max-w-3xl text-base leading-relaxed text-muted sm:text-lg">{m.compareIntro()}</p>

		<div class="-mx-4 mt-10 grid gap-px border-y border-line bg-line sm:-mx-8 lg:grid-cols-2">
			{#each groups as group (group.id)}
				<figure class="bg-bg p-6 sm:p-8" aria-labelledby="{group.id}-title">
					<figcaption>
						<h3 id="{group.id}-title" class="text-[18px]">{group.title}</h3>
						<p class="mt-1 text-sm text-muted">{m.compareLower()}</p>
					</figcaption>
					{#each group.metrics as metric (metric.label)}
						{@const max = Math.max(...metric.rows.map((row) => row.value))}
						<div class="mt-6">
							<p class="text-sm font-medium text-fg">{metric.label}</p>
							<dl class="mt-3 space-y-3">
								{#each metric.rows as row (row.name)}
									<div
										class="grid grid-cols-[7.5rem_1fr] items-center gap-3 text-sm sm:grid-cols-[9rem_1fr]"
									>
										<dt class="truncate text-muted">{row.name}</dt>
										<dd class="flex items-center gap-3">
											<span class="block h-2 min-w-0 flex-1 overflow-hidden rounded-full bg-line">
												<span
													class={`block h-full rounded-full ${row.ours ? 'bg-rust-600 dark:bg-rust-400' : 'bg-bar-other'}`}
													style={`width: ${(row.value / max) * 100}%`}
												></span>
											</span>
											<span class="w-20 shrink-0 text-right font-mono tabular-nums">{row.text}</span
											>
										</dd>
									</div>
								{/each}
							</dl>
						</div>
					{/each}
					{#if group.id === 'memory'}
						<p class="mt-6 text-sm leading-relaxed text-muted">{m.compareMemoryNote()}</p>
					{/if}
				</figure>
			{/each}
		</div>

		<p class="mt-8 max-w-3xl text-sm leading-relaxed text-muted">
			{m.compareOfficial()}
			{m.compareSources()}
			{#each sources as source, index (source.href)}
				<a
					href={source.href}
					rel="external"
					class="text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
					>{source.label}</a
				>{index < sources.length - 1 ? ', ' : '.'}
			{/each}
		</p>

		<div
			class="mt-6 max-w-3xl rounded-xl border border-line p-4 text-xs leading-relaxed text-muted"
		>
			<p class="font-medium text-fg">{m.compareMethodTitle()}</p>
			<p class="mt-1">{m.compareMethod({ date })}</p>
		</div>
	</div>
</section>
