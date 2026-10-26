<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import qr from '#lib/assets/screenshot-qr-linux.webp';
	import tray from '#lib/assets/screenshot-tray-linux.webp';
</script>

<section
	id="screenshots"
	aria-labelledby="screenshots-title"
	class="border-y border-stone-200 bg-stone-100/60 dark:border-stone-800 dark:bg-stone-900/40"
>
	<div class="mx-auto max-w-6xl px-4 py-16 sm:px-6 sm:py-20">
		<h2 id="screenshots-title" class="text-3xl font-bold tracking-tight sm:text-4xl">
			{m.screenshotsTitle()}
		</h2>
		<p class="mt-3 text-stone-600 dark:text-stone-400">{m.screenshotsIntro()}</p>
		<div class="mt-10 grid items-start gap-6 lg:grid-cols-3">
			<figure class="lg:col-span-2">
				<img
					src={qr}
					alt={m.shotQrAlt()}
					width="1200"
					height="862"
					loading="lazy"
					decoding="async"
					class="h-auto w-full rounded-xl border border-stone-200 shadow-lg dark:border-stone-800"
				/>
				<figcaption class="mt-3 text-sm text-stone-600 dark:text-stone-400">
					{m.shotQrCaption()}
				</figcaption>
			</figure>
			<figure>
				<img
					src={tray}
					alt={m.shotTrayAlt()}
					width="760"
					height="200"
					loading="lazy"
					decoding="async"
					class="h-auto w-full rounded-xl border border-stone-200 shadow-lg dark:border-stone-800"
				/>
				<figcaption class="mt-3 text-sm text-stone-600 dark:text-stone-400">
					{m.shotTrayCaption()}
				</figcaption>
			</figure>
		</div>
	</div>
</section>
