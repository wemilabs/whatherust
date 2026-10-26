<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '#lib/paraglide/messages.js';
	import { detectPlatform, pickAssets, pickPrimary, type Platform } from '#lib/downloads.js';
	import type { Release } from '#lib/types.js';

	let { latest }: { latest: Pick<Release, 'tag' | 'assets'> | null } = $props();

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

	const picked = $derived(pickAssets(latest));
	const options = $derived(
		[
			{ label: m.assetWindowsSetup(), asset: picked.windows.setup },
			{ label: m.assetWindowsMsi(), asset: picked.windows.msi },
			{ label: m.assetMacDmg(), asset: picked.macos.dmg },
			{ label: m.assetLinuxAppImage(), asset: picked.linux.appimage },
			{ label: m.assetLinuxDeb(), asset: picked.linux.deb },
			{ label: m.assetLinuxRpm(), asset: picked.linux.rpm }
		].filter((option) => option.asset)
	);
	const primary = $derived(pickPrimary(platform, latest));
	const others = $derived(options.filter((option) => option.asset?.url !== primary?.asset.url));

	function size(bytes: number) {
		return `${(bytes / 1_000_000).toFixed(1)} MB`;
	}

	function primaryLabel(kind: 'setup' | 'dmg' | 'appimage') {
		if (kind === 'setup') return m.assetWindowsSetup();
		if (kind === 'dmg') return m.assetMacDmg();
		return m.assetLinuxAppImage();
	}
</script>

<div class="flex flex-col items-center gap-3 sm:items-start">
	{#if latest && primary && currentOs}
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
	{:else if !latest}
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

	{#if others.length > 0}
		<details class="group w-full max-w-md text-sm" open={!primary}>
			<summary
				class="cursor-pointer rounded-md py-1 font-medium text-stone-700 select-none hover:text-stone-900 dark:text-stone-300 dark:hover:text-white"
			>
				{primary ? m.otherDownloads() : m.allDownloads()}
			</summary>
			<ul
				class="mt-2 divide-y divide-stone-200 overflow-hidden rounded-xl border border-stone-200 bg-white text-left dark:divide-stone-800 dark:border-stone-800 dark:bg-stone-900"
			>
				{#each others as option (option.asset?.url)}
					<li>
						<a
							href={option.asset?.url}
							download
							rel="external"
							class="flex items-center justify-between gap-4 px-4 py-2.5 hover:bg-stone-100 dark:hover:bg-stone-800"
						>
							<span>{option.label}</span>
							<span class="text-stone-500 tabular-nums dark:text-stone-400"
								>{size(option.asset?.size ?? 0)}</span
							>
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
