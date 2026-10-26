<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import qr from '#lib/assets/screenshot-qr-linux.webp';
	import tray from '#lib/assets/screenshot-tray-linux.webp';
</script>

<section id="screenshots" aria-labelledby="screenshots-title" class="border-t border-line">
	<div class="px-4 py-16 sm:px-8 sm:py-20">
		<p class="font-mono text-xs text-muted tabular-nums">02</p>
		<h2 id="screenshots-title" class="mt-5 text-[32px] sm:text-[40px]">
			<span class="text-subtle">{m.screenshotsLead()}</span>
			<span class="block">{m.screenshotsTitle()}</span>
		</h2>
		<p class="mt-5 max-w-2xl text-base leading-relaxed text-muted sm:text-lg">
			{m.screenshotsIntro()}
		</p>
		<div class="mt-10 grid items-start gap-8 lg:grid-cols-3">
			<figure class="lg:col-span-2">
				<div class="rounded-xl border border-line bg-surface p-2">
					<img
						src={qr}
						alt={m.shotQrAlt()}
						width="1200"
						height="862"
						loading="lazy"
						decoding="async"
						class="h-auto w-full rounded-lg"
					/>
				</div>
				<figcaption class="mt-3 text-sm text-muted">{m.shotQrCaption()}</figcaption>
			</figure>
			<figure>
				<div class="rounded-xl border border-line bg-surface p-2">
					<img
						src={tray}
						alt={m.shotTrayAlt()}
						width="760"
						height="200"
						loading="lazy"
						decoding="async"
						class="h-auto w-full rounded-lg"
					/>
				</div>
				<figcaption class="mt-3 text-sm text-muted">{m.shotTrayCaption()}</figcaption>
			</figure>
		</div>
	</div>
</section>
