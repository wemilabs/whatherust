<script lang="ts">
	import type { Path } from '$app/types';
	import { resolve } from '$app/paths';
	import { getLocale, localizeHref } from '#lib/paraglide/runtime.js';
	import { m } from '#lib/paraglide/messages.js';
	import { RELEASES_URL, REPO_URL } from '#lib/site.js';

	const locale = $derived(getLocale());
	const href = (path: string) => resolve(localizeHref(path, { locale }) as Path);
</script>

<footer class="border-t border-line">
	<div class="mx-auto grid max-w-6xl gap-12 px-4 py-12 sm:px-8 lg:grid-cols-[1fr_auto]">
		<a
			href={href('/')}
			class="flex h-fit items-center gap-2 text-[19px] font-medium tracking-[-0.02em]"
		>
			<img src="/favicon.svg" alt="" width="28" height="28" class="size-7" />
			<span>WhatheRust</span>
		</a>
		<div class="grid grid-cols-2 gap-12 text-sm">
			<div>
				<h2 class="text-sm font-medium text-fg">{m.footerProject()}</h2>
				<ul class="mt-4 space-y-2 text-muted">
					<li><a href={REPO_URL} rel="external" class="hover:text-fg">{m.footerSource()}</a></li>
					<li>
						<a href={RELEASES_URL} rel="external" class="hover:text-fg">{m.footerReleases()}</a>
					</li>
					<li><a href={href('/changelog')} class="hover:text-fg">{m.footerChangelog()}</a></li>
					<li>
						<a href={`${REPO_URL}/blob/main/LICENSE`} rel="external" class="hover:text-fg"
							>{m.footerLicense()}</a
						>
					</li>
				</ul>
			</div>
			<div>
				<h2 class="text-sm font-medium text-fg">{m.footerResources()}</h2>
				<ul class="mt-4 space-y-2 text-muted">
					<li>
						<a href="https://tauri.app" rel="external" class="hover:text-fg">{m.footerTauri()}</a>
					</li>
					<li>
						<a href="https://www.rust-lang.org" rel="external" class="hover:text-fg"
							>{m.footerRust()}</a
						>
					</li>
				</ul>
			</div>
		</div>
		<div
			class="col-span-full flex flex-col gap-2 border-t border-line pt-6 text-xs text-muted sm:flex-row sm:justify-between"
		>
			<p>{m.disclaimer()}</p>
			<p>{m.footerBuilt()}</p>
		</div>
	</div>
</footer>
