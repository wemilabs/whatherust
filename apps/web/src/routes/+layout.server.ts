import { getReleases, getRepo } from '#lib/server/github.js';
import type { LayoutServerLoad } from './$types';

export const prerender = false;

export const load: LayoutServerLoad = async ({ fetch, setHeaders }) => {
	// Cached on Vercel's CDN for every locale (the /fr routes are rerouted, so route-level ISR would miss them).
	setHeaders({ 'cache-control': 'public, max-age=0, s-maxage=600, stale-while-revalidate=86400' });

	const [repo, releases] = await Promise.all([getRepo(fetch), getReleases(fetch)]);
	const latestRelease = releases?.find((release) => !release.prerelease) ?? null;
	const latest =
		releases === null
			? undefined
			: latestRelease && {
					tag: latestRelease.tag,
					url: latestRelease.url,
					assets: latestRelease.assets
				};

	return {
		stars: repo?.stars ?? null,
		latest
	};
};
