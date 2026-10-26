<script lang="ts">
	import { onMount } from 'svelte';
	import { m } from '#lib/paraglide/messages.js';
	import { detectPlatform, pickAssets, type Platform } from '#lib/downloads.js';
	import { RELEASES_URL } from '#lib/site.js';
	import type { Release } from '#lib/types.js';

	let { latest }: { latest: Pick<Release, 'tag' | 'url' | 'assets'> | null } = $props();

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

	const picked = $derived(pickAssets(latest));
	const options = $derived(
		[
			{ os: 'windows', label: m.assetWindowsSetup(), asset: picked.windows.setup },
			{ os: 'windows', label: m.assetWindowsMsi(), asset: picked.windows.msi },
			{ os: 'macos', label: m.assetMacDmg(), asset: picked.macos.dmg },
			{ os: 'linux', label: m.assetLinuxAppImage(), asset: picked.linux.appimage },
			{ os: 'linux', label: m.assetLinuxDeb(), asset: picked.linux.deb },
			{ os: 'linux', label: m.assetLinuxRpm(), asset: picked.linux.rpm }
		].filter((option) => option.asset)
	);
	const primary = $derived(
		platform === 'unknown' ? undefined : options.find((option) => option.os === platform)
	);
	const others = $derived(options.filter((option) => option !== primary));

	function size(bytes: number) {
		return `${(bytes / 1_000_000).toFixed(1)} MB`;
	}
</script>

<div class="flex flex-col items-center gap-3 sm:items-start">
	{#if primary?.asset && platform !== 'unknown'}
		<a
			href={primary.asset.url}
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
		<p class="text-sm text-stone-600 dark:text-stone-400">
			{m.downloadVersion({ version: latest?.tag ?? '' })} · {primary.label} · {size(
				primary.asset.size
			)}
		</p>
	{:else}
		<a
			href={latest?.url ?? RELEASES_URL}
			rel="external"
			class="inline-flex min-h-12 items-center gap-2 rounded-xl bg-wa-700 px-6 py-3 text-base font-semibold text-white shadow-sm transition-colors hover:bg-wa-800"
		>
			{m.downloadReleases()}
		</a>
		{#if !latest}
			<p class="text-sm text-stone-600 dark:text-stone-400">{m.noReleases()}</p>
		{/if}
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
