import { GITHUB_TOKEN } from '$app/env/private';
import type { Release } from '#lib/types.js';

type GitHubRelease = {
	draft: boolean;
	tag_name: string;
	name: string | null;
	html_url: string;
	published_at: string | null;
	prerelease: boolean;
	body: string | null;
	assets: {
		name: string;
		browser_download_url: string;
		size: number;
	}[];
};

const API_URL = 'https://api.github.com';

function headers(): Record<string, string> {
	const result = {
		Accept: 'application/vnd.github+json',
		'X-GitHub-Api-Version': '2022-11-28',
		'User-Agent': 'whatherust-site'
	};

	return GITHUB_TOKEN ? { ...result, Authorization: `Bearer ${GITHUB_TOKEN}` } : result;
}

async function request<T>(
	url: string,
	fetcher: typeof fetch
): Promise<{ data: T; headers: Headers } | null> {
	try {
		const response = await fetcher(url, { headers: headers() });
		if (!response.ok) return null;
		return { data: (await response.json()) as T, headers: response.headers };
	} catch {
		return null;
	}
}

export async function getRepo(fetcher: typeof fetch): Promise<{ stars: number } | null> {
	const response = await request<{ stargazers_count: number }>(
		`${API_URL}/repos/wemilabs/whatherust`,
		fetcher
	);
	const repo = response?.data;

	return repo && typeof repo.stargazers_count === 'number'
		? { stars: repo.stargazers_count }
		: null;
}

function nextPage(headers: Headers): string | null {
	for (const link of headers.get('Link')?.split(',') ?? []) {
		const url = link.match(/<([^>]+)>/)?.[1];
		if (url && /;\s*rel="?next"?\s*(?:;|$)/i.test(link)) return url;
	}

	return null;
}

export async function getReleases(
	fetcher: typeof fetch,
	{ all = false }: { all?: boolean } = {}
): Promise<Release[]> {
	const response = await request<GitHubRelease[]>(
		`${API_URL}/repos/wemilabs/whatherust/releases?per_page=${all ? 100 : 20}`,
		fetcher
	);

	if (!response || !Array.isArray(response.data)) return [];

	const releases = [...response.data];
	if (all) {
		let next = nextPage(response.headers);
		for (let page = 1; page < 10 && next; page += 1) {
			if (!next.startsWith(`${API_URL}/`)) break;
			const nextResponse = await request<GitHubRelease[]>(next, fetcher);
			if (!nextResponse || !Array.isArray(nextResponse.data)) break;
			releases.push(...nextResponse.data);
			next = nextPage(nextResponse.headers);
		}
	}

	return releases
		.filter((release) => !release.draft)
		.map((release) => ({
			tag: release.tag_name,
			name: release.name ?? release.tag_name,
			url: release.html_url,
			publishedAt: release.published_at,
			prerelease: release.prerelease,
			body: release.body ?? '',
			assets: release.assets.map((asset) => ({
				name: asset.name,
				url: asset.browser_download_url,
				size: asset.size
			}))
		}));
}
