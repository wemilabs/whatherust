<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import {
		formatMB,
		macosMeasurements,
		measurements,
		windowsMeasurements
	} from '#lib/comparison.js';

	const { whatherust: linuxWhatheRust, electron: linuxElectron } = measurements;
	const { whatherust: macWhatheRust, electron: macElectron } = macosMeasurements;
	const { whatherust: windowsWhatheRust, electron: windowsElectron } = windowsMeasurements;

	const sizePair = (
		label: string,
		oursName: string,
		oursValue: number,
		electronName: string,
		electronValue: number
	) => ({
		label,
		rows: [
			{ name: oursName, value: oursValue, text: formatMB(oursValue), ours: true },
			{ name: electronName, value: electronValue, text: formatMB(electronValue), ours: false }
		]
	});

	const platforms = $derived([
		{
			id: 'ubuntu',
			title: m.compareUbuntuTitle(),
			size: [
				sizePair(
					m.compareSizeDeb(),
					linuxWhatheRust.name,
					linuxWhatheRust.deb,
					linuxElectron.name,
					linuxElectron.deb
				),
				sizePair(
					m.compareSizeAppImage(),
					linuxWhatheRust.name,
					linuxWhatheRust.appimage,
					linuxElectron.name,
					linuxElectron.appimage
				)
			]
		},
		{
			id: 'macos',
			title: m.compareMacTitle(),
			size: [
				sizePair(
					m.compareMacSizeDmg(),
					macWhatheRust.name,
					macWhatheRust.dmgArm64,
					macElectron.name,
					macElectron.dmgArm64
				),
				sizePair(
					m.compareMacSizeInstalled(),
					macWhatheRust.name,
					macWhatheRust.installedApp,
					macElectron.name,
					macElectron.installedApp
				)
			]
		},
		{
			id: 'windows',
			title: m.compareWindowsTitle(),
			size: [
				sizePair(
					m.compareWindowsSizeInstaller(),
					windowsWhatheRust.name,
					windowsWhatheRust.installer,
					windowsElectron.name,
					windowsElectron.installer
				),
				sizePair(
					m.compareWindowsSizeInstalled(),
					windowsWhatheRust.name,
					windowsWhatheRust.installedApp,
					windowsElectron.name,
					windowsElectron.installedApp
				)
			]
		}
	]);
</script>

<section id="comparison" aria-labelledby="comparison-title" class="border-t border-line">
	<div class="px-4 py-16 sm:px-8 sm:py-20">
		<p class="font-mono text-xs text-muted tabular-nums">03</p>
		<h2 id="comparison-title" class="mt-5 text-[32px] sm:text-[40px]">{m.compareTitle()}</h2>
		<p class="mt-5 max-w-3xl text-base leading-relaxed text-muted sm:text-lg">{m.compareIntro()}</p>

		<div
			class="-mx-4 mt-10 grid grid-cols-1 gap-px border-y border-line bg-line sm:-mx-8 lg:grid-cols-3"
		>
			{#each platforms as platform (platform.id)}
				<section aria-labelledby="{platform.id}-title" class="h-full bg-bg p-6 sm:p-8">
					<h3 id="{platform.id}-title" class="text-[18px] font-medium tracking-[-0.01em]">
						{platform.title}
					</h3>
					<p class="mt-2 text-sm text-muted">
						{m.compareSize()} <span aria-hidden="true">·</span>
						{m.compareLower()}
					</p>
					<div class="mt-6 space-y-6">
						{#each platform.size as metric (metric.label)}
							{@const max = Math.max(...metric.rows.map((row) => row.value))}
							<div>
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
												<span class="w-20 shrink-0 text-right font-mono tabular-nums">
													{row.text}
												</span>
											</dd>
										</div>
									{/each}
								</dl>
							</div>
						{/each}
					</div>
				</section>
			{/each}
		</div>

		<p class="mt-8 max-w-3xl text-sm leading-relaxed text-muted">
			{m.compareOfficialIntro()}
			<a
				href="https://x.com/mthlish/status/2114650982663758226"
				target="_blank"
				rel="external noopener noreferrer"
				class="text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
				>{m.compareOfficialSourceX()}</a
			>,
			<a
				href="https://www.windowslatest.com/2025/07/21/whatsapp-for-windows-11-is-switching-back-to-chromium-web-wrapper-from-uwp-native/"
				target="_blank"
				rel="external noopener noreferrer"
				class="text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
				>{m.compareOfficialSourceWindowsLatest()}</a
			>,
			<a
				href="https://daringfireball.net/2025/11/meta_whatsapp_windows_shitty_web_app"
				target="_blank"
				rel="external noopener noreferrer"
				class="text-rust-700 underline decoration-transparent underline-offset-4 hover:decoration-current dark:text-rust-400"
				>{m.compareOfficialSourceDaringFireball()}</a
			>.
		</p>
	</div>
</section>
