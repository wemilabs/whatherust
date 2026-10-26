<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { RELEASES_URL, REPO_URL } from '#lib/site.js';
</script>

<footer class="border-t border-stone-200 dark:border-stone-800">
	<div
		class="mx-auto flex max-w-6xl flex-col gap-4 px-4 py-10 text-sm text-stone-600 sm:px-6 dark:text-stone-400"
	>
		<div class="flex flex-wrap gap-x-6 gap-y-2">
			<a
				href={REPO_URL}
				rel="external"
				class="rounded underline-offset-4 hover:text-stone-900 hover:underline dark:hover:text-white"
				>{m.footerSource()}</a
			>
			<a
				href={RELEASES_URL}
				rel="external"
				class="rounded underline-offset-4 hover:text-stone-900 hover:underline dark:hover:text-white"
				>{m.footerReleases()}</a
			>
		</div>
		<p>{m.disclaimer()}</p>
		<p>{m.footerBuilt()}</p>
	</div>
</footer>
