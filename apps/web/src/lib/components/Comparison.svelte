<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { getLocale } from '#lib/paraglide/runtime.js';
	import {
		formatMB,
		formatMiB,
		macosMeasurements,
		measuredOn,
		measurements
	} from '#lib/comparison.js';

	const { whatherust: linuxWhatheRust, electron: linuxElectron } = measurements;
	const { whatherust: macWhatheRust, electron: macElectron } = macosMeasurements;

	const date = $derived(
		new Intl.DateTimeFormat(getLocale(), { dateStyle: 'long', timeZone: 'UTC' }).format(
			new Date(measuredOn)
		)
	);

	const platforms = $derived([
		{
			id: 'ubuntu',
			title: m.compareUbuntuTitle(),
			size: [
				{
					label: m.compareSizeDeb(),
					rows: [
						{
							name: linuxWhatheRust.name,
							value: linuxWhatheRust.deb,
							text: formatMB(linuxWhatheRust.deb),
							detail: '',
							ours: true
						},
						{
							name: linuxElectron.name,
							value: linuxElectron.deb,
							text: formatMB(linuxElectron.deb),
							detail: '',
							ours: false
						}
					]
				},
				{
					label: m.compareSizeAppImage(),
					rows: [
						{
							name: linuxWhatheRust.name,
							value: linuxWhatheRust.appimage,
							text: formatMB(linuxWhatheRust.appimage),
							detail: '',
							ours: true
						},
						{
							name: linuxElectron.name,
							value: linuxElectron.appimage,
							text: formatMB(linuxElectron.appimage),
							detail: '',
							ours: false
						}
					]
				}
			],
			memory: [
				{
					label: 'RSS',
					rows: [
						{
							name: linuxWhatheRust.name,
							value: linuxWhatheRust.rssMiB,
							text: formatMiB(linuxWhatheRust.rssMiB),
							detail: '',
							ours: true
						},
						{
							name: linuxElectron.name,
							value: linuxElectron.rssMiB,
							text: formatMiB(linuxElectron.rssMiB),
							detail: '',
							ours: false
						}
					]
				}
			],
			memoryNote: m.compareLinuxMemoryNote(),
			method: m.compareMethod({ date })
		},
		{
			id: 'macos',
			title: m.compareMacTitle(),
			size: [
				{
					label: m.compareMacSizeDmg(),
					rows: [
						{
							name: macWhatheRust.name,
							value: macWhatheRust.dmgArm64,
							text: formatMB(macWhatheRust.dmgArm64),
							detail: '',
							ours: true
						},
						{
							name: macElectron.name,
							value: macElectron.dmgArm64,
							text: formatMB(macElectron.dmgArm64),
							detail: '',
							ours: false
						}
					]
				},
				{
					label: m.compareMacSizeInstalled(),
					rows: [
						{
							name: macWhatheRust.name,
							value: macWhatheRust.installedApp,
							text: formatMB(macWhatheRust.installedApp),
							detail: '',
							ours: true
						},
						{
							name: macElectron.name,
							value: macElectron.installedApp,
							text: formatMB(macElectron.installedApp),
							detail: '',
							ours: false
						}
					]
				}
			],
			memory: [
				{
					label: 'RSS',
					rows: [
						{
							name: macWhatheRust.name,
							value: macWhatheRust.rssMedianMiB,
							text: formatMiB(macWhatheRust.rssMedianMiB),
							detail: `runs: ${Math.round(macWhatheRust.rssRunsMiB[0])} / ${Math.round(macWhatheRust.rssRunsMiB[1])}`,
							ours: true
						},
						{
							name: macElectron.name,
							value: macElectron.rssMedianMiB,
							text: formatMiB(macElectron.rssMedianMiB),
							detail: `runs: ${Math.round(macElectron.rssRunsMiB[0])} / ${Math.round(macElectron.rssRunsMiB[1])}`,
							ours: false
						}
					]
				}
			],
			memoryNote: m.compareMacMemoryNote(),
			method: m.compareMacMethod()
		}
	]);

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

		<div class="mt-10 space-y-10">
			{#each platforms as platform (platform.id)}
				<section aria-labelledby="{platform.id}-title">
					<h3 id="{platform.id}-title" class="mb-4 text-[18px] font-medium tracking-[-0.01em]">
						{platform.title}
					</h3>
					<div class="-mx-4 grid gap-px border-y border-line bg-line sm:-mx-8 lg:grid-cols-2">
						<div class="bg-bg p-6 sm:p-8">
							<h4 class="text-[18px]">{m.compareSize()}</h4>
							<p class="mt-1 text-sm text-muted">{m.compareLower()}</p>
							{#each platform.size as metric (metric.label)}
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
													<span
														class="block h-2 min-w-0 flex-1 overflow-hidden rounded-full bg-line"
													>
														<span
															class={`block h-full rounded-full ${row.ours ? 'bg-rust-600 dark:bg-rust-400' : 'bg-bar-other'}`}
															style={`width: ${(row.value / max) * 100}%`}
														></span>
													</span>
													<span class="w-20 shrink-0 text-right font-mono tabular-nums">
														<span class="block">{row.text}</span>
														{#if row.detail}
															<span
																class="mt-0.5 block text-[10px] leading-tight whitespace-nowrap text-subtle"
																>{row.detail}</span
															>
														{/if}
													</span>
												</dd>
											</div>
										{/each}
									</dl>
								</div>
							{/each}
						</div>
						<div class="bg-bg p-6 sm:p-8">
							<h4 class="text-[18px]">{m.compareMemory()}</h4>
							<p class="mt-1 text-sm text-muted">{m.compareLower()}</p>
							{#each platform.memory as metric (metric.label)}
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
													<span
														class="block h-2 min-w-0 flex-1 overflow-hidden rounded-full bg-line"
													>
														<span
															class={`block h-full rounded-full ${row.ours ? 'bg-rust-600 dark:bg-rust-400' : 'bg-bar-other'}`}
															style={`width: ${(row.value / max) * 100}%`}
														></span>
													</span>
													<span class="w-20 shrink-0 text-right font-mono tabular-nums">
														<span class="block">{row.text}</span>
														{#if row.detail}
															<span
																class="mt-0.5 block text-[10px] leading-tight whitespace-nowrap text-subtle"
																>{row.detail}</span
															>
														{/if}
													</span>
												</dd>
											</div>
										{/each}
									</dl>
								</div>
							{/each}
							<p class="mt-6 text-sm leading-relaxed text-muted">{platform.memoryNote}</p>
						</div>
					</div>
					<div
						class="mt-6 max-w-3xl rounded-xl border border-line p-4 text-xs leading-relaxed text-muted"
					>
						<p class="font-medium text-fg">{m.compareMethodTitle()}</p>
						<p class="mt-1">{platform.method}</p>
					</div>
				</section>
			{/each}
		</div>

		<p class="mt-6 text-sm text-muted">{m.compareWindowsNote()}</p>

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
	</div>
</section>
