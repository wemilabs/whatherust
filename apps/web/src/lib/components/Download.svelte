<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '#lib/paraglide/messages.js';
	import {
		detectPlatform,
		pickAssets,
		pickPrimary,
		STABLE_DOWNLOADS,
		type Platform
	} from '#lib/downloads.js';
	import type { Release } from '#lib/types.js';

	type DownloadOption = {
		platform: Exclude<Platform, 'unknown'>;
		label: string;
		url: string;
		size?: number;
	};

	let { latest }: { latest: Pick<Release, 'tag' | 'assets'> | null | undefined } = $props();

	let platform = $state<Platform>('unknown');
	onMount(() => {
		platform = detectPlatform(
			navigator as Navigator & { userAgentData?: { platform?: string; mobile?: boolean } }
		);
	});

	const osName: Record<Exclude<Platform, 'unknown'>, string> = {
		windows: 'Windows',
		macos: 'macOS',
		linux: 'Linux'
	};
	const currentOs = $derived(platform === 'unknown' ? undefined : osName[platform]);

	const primary = $derived(latest ? pickPrimary(platform, latest) : undefined);
	const hasPrimary = $derived(latest === undefined ? platform !== 'unknown' : Boolean(primary));
	const options = $derived.by((): DownloadOption[] => {
		if (latest === undefined) {
			const stableOptions: DownloadOption[] = [
				{
					platform: 'windows',
					label: m.assetWindowsSetup(),
					url: STABLE_DOWNLOADS.windows
				},
				{ platform: 'macos', label: m.assetMacDmg(), url: STABLE_DOWNLOADS.macos },
				{
					platform: 'linux',
					label: m.assetLinuxAppImage(),
					url: STABLE_DOWNLOADS.linux
				}
			];
			return stableOptions.filter(
				(option) => platform === 'unknown' || option.platform !== platform
			);
		}

		const picked = pickAssets(latest);
		const available = [
			{ platform: 'windows' as const, label: m.assetWindowsSetup(), asset: picked.windows.setup },
			{ platform: 'windows' as const, label: m.assetWindowsMsi(), asset: picked.windows.msi },
			{ platform: 'macos' as const, label: m.assetMacDmg(), asset: picked.macos.dmg },
			{ platform: 'linux' as const, label: m.assetLinuxAppImage(), asset: picked.linux.appimage },
			{ platform: 'linux' as const, label: m.assetLinuxDeb(), asset: picked.linux.deb },
			{ platform: 'linux' as const, label: m.assetLinuxRpm(), asset: picked.linux.rpm }
		];

		return available.flatMap(({ platform: optionPlatform, label, asset }) =>
			asset && asset.url !== primary?.asset.url
				? [{ platform: optionPlatform, label, url: asset.url, size: asset.size }]
				: []
		);
	});

	function size(bytes: number) {
		return `${(bytes / 1_000_000).toFixed(1)} MB`;
	}

	function primaryLabel(kind: 'setup' | 'msi' | 'dmg' | 'appimage' | 'deb' | 'rpm') {
		if (kind === 'setup') return m.assetWindowsSetup();
		if (kind === 'msi') return m.assetWindowsMsi();
		if (kind === 'dmg') return m.assetMacDmg();
		if (kind === 'appimage') return m.assetLinuxAppImage();
		if (kind === 'deb') return m.assetLinuxDeb();
		return m.assetLinuxRpm();
	}
</script>

<div class="flex flex-col items-center gap-3 sm:items-start">
	{#if latest === undefined && platform !== 'unknown'}
		<a
			href={STABLE_DOWNLOADS[platform]}
			download
			rel="external"
			class="inline-flex min-h-12 items-center gap-2 rounded-xl bg-wa-700 px-6 py-3 text-base font-semibold text-white shadow-sm transition-colors hover:bg-wa-800"
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
				<path d="M12 3v12m0 0-4-4m4 4 4-4M5 21h14" />
			</svg>
			{m.downloadFor({ os: osName[platform] })}
		</a>
	{:else if latest && primary && currentOs}
		<a
			href={primary.asset.url}
			download
			rel="external"
			class="inline-flex min-h-12 items-center gap-2 rounded-xl bg-wa-700 px-6 py-3 text-base font-semibold text-white shadow-sm transition-colors hover:bg-wa-800"
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
				<path d="M12 3v12m0 0-4-4m4 4 4-4M5 21h14" />
			</svg>
			{m.downloadFor({ os: currentOs })}
		</a>
		<p class="text-sm text-stone-600 dark:text-stone-400">
			{m.downloadVersion({ version: latest.tag })} · {primaryLabel(primary.kind)} · {size(
				primary.asset.size
			)}
		</p>
	{:else if latest === null}
		<button
			disabled
			class="inline-flex min-h-12 items-center gap-2 rounded-xl bg-wa-700 px-6 py-3 text-base font-semibold text-white opacity-60 shadow-sm"
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
				<path d="M12 3v12m0 0-4-4m4 4 4-4M5 21h14" />
			</svg>
			{currentOs ? m.downloadFor({ os: currentOs }) : m.download()}
		</button>
		<p class="text-sm text-stone-600 dark:text-stone-400">{m.noReleases()}</p>
	{/if}

	{#if options.length > 0}
		<details class="group w-full max-w-md text-sm" open={!hasPrimary}>
			<summary
				class="cursor-pointer rounded-md py-1 font-medium text-stone-700 select-none hover:text-stone-900 dark:text-stone-300 dark:hover:text-white"
			>
				{hasPrimary ? m.otherDownloads() : m.allDownloads()}
			</summary>
			<ul
				class="mt-2 divide-y divide-stone-200 overflow-hidden rounded-xl border border-stone-200 bg-white text-left dark:divide-stone-800 dark:border-stone-800 dark:bg-stone-900"
			>
				{#each options as option (option.url)}
					<li>
						<a
							href={option.url}
							download
							rel="external"
							class="flex items-center justify-between gap-4 px-4 py-2.5 hover:bg-stone-100 dark:hover:bg-stone-800"
						>
							<span>{option.label}</span>
							{#if option.size !== undefined}
								<span class="text-stone-500 tabular-nums dark:text-stone-400"
									>{size(option.size)}</span
								>
							{/if}
						</a>
					</li>
				{/each}
			</ul>
		</details>
	{/if}

	<p class="max-w-md text-center text-xs text-stone-500 sm:text-left dark:text-stone-400">
		{m.unsigned()}
	</p>
</div>
