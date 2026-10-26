import { renderMarkdown } from '#lib/server/markdown.js';
import { getReleases } from '#lib/server/github.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const releases = await getReleases(fetch, { all: true });

	return {
		releases: releases.map(({ tag, name, url, publishedAt, prerelease, body }) => ({
			tag,
			name,
			url,
			publishedAt,
			prerelease,
			html: renderMarkdown(body)
		}))
	};
};
