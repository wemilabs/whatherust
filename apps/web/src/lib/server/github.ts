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

async function request<T>(path: string, fetcher: typeof fetch): Promise<T | null> {
	try {
		const response = await fetcher(`${API_URL}${path}`, { headers: headers() });
		if (!response.ok) return null;
		return (await response.json()) as T;
	} catch {
		return null;
	}
}

export async function getRepo(fetcher: typeof fetch): Promise<{ stars: number } | null> {
	const repo = await request<{ stargazers_count: number }>('/repos/wemilabs/whatherust', fetcher);

	return repo && typeof repo.stargazers_count === 'number'
		? { stars: repo.stargazers_count }
		: null;
}

export async function getReleases(fetcher: typeof fetch): Promise<Release[]> {
	const releases = await request<GitHubRelease[]>(
		'/repos/wemilabs/whatherust/releases?per_page=20',
		fetcher
	);

	if (!Array.isArray(releases)) return [];

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
