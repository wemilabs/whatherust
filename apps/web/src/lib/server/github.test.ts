import { beforeEach, describe, expect, it, vi } from 'vitest';
import { getReleases } from './github';

const mockEnv = vi.hoisted(() => ({ githubToken: '' }));

vi.mock('$app/env/private', () => ({
	get GITHUB_TOKEN() {
		return mockEnv.githubToken;
	}
}));

const apiUrl = 'https://api.github.com/repos/wemilabs/whatherust/releases';
const nextPageUrl = `${apiUrl}?per_page=100&page=2`;

function githubRelease(tag: string, draft = false) {
	return {
		draft,
		tag_name: tag,
		name: null,
		html_url: `https://github.com/wemilabs/whatherust/releases/tag/${tag}`,
		published_at: null,
		prerelease: false,
		body: null,
		assets: []
	};
}

function makeFetcher(
	responses: Array<{ body: unknown; headers?: Record<string, string>; status?: number }>
) {
	const calls: Array<{ url: string; init?: RequestInit }> = [];
	const fetcher: typeof fetch = async (input, init) => {
		calls.push({ url: String(input), init });
		const response = responses.shift();
		if (!response) throw new Error('Unexpected fetch call');
		return new Response(JSON.stringify(response.body), {
			status: response.status ?? 200,
			headers: response.headers
		});
	};

	return { fetcher, calls };
}

beforeEach(() => {
	mockEnv.githubToken = '';
});

describe('getReleases', () => {
	it('returns null when the first page request fails', async () => {
		const { fetcher } = makeFetcher([{ body: {}, status: 503 }]);

		await expect(getReleases(fetcher)).resolves.toBeNull();
	});

	it('returns null when the first page response is not an array', async () => {
		const { fetcher } = makeFetcher([{ body: {} }]);

		await expect(getReleases(fetcher)).resolves.toBeNull();
	});

	it('returns null when the first page fetch throws', async () => {
		const fetcher: typeof fetch = async () => {
			throw new Error('network failure');
		};

		await expect(getReleases(fetcher)).resolves.toBeNull();
	});

	it('returns an empty array when GitHub has no releases', async () => {
		const { fetcher } = makeFetcher([{ body: [] }]);

		await expect(getReleases(fetcher)).resolves.toEqual([]);
	});

	it('requests one page of 20 releases by default', async () => {
		const { fetcher, calls } = makeFetcher([
			{ body: [githubRelease('v0.1.0')], headers: { Link: `<${nextPageUrl}>; rel="next"` } }
		]);

		const releases = await getReleases(fetcher);

		expect(calls).toHaveLength(1);
		expect(calls[0].url).toBe(`${apiUrl}?per_page=20`);
		expect(releases?.map((release) => release.tag)).toEqual(['v0.1.0']);
	});

	it('follows two pages when all releases are requested', async () => {
		const { fetcher, calls } = makeFetcher([
			{ body: [githubRelease('v0.2.0')], headers: { Link: `<${nextPageUrl}>; rel="next"` } },
			{ body: [githubRelease('v0.1.0')] }
		]);

		const releases = await getReleases(fetcher, { all: true });

		expect(calls.map((call) => call.url)).toEqual([`${apiUrl}?per_page=100`, nextPageUrl]);
		expect(calls[0].init?.redirect).toBeUndefined();
		expect(calls[1].init?.redirect).toBe('error');
		expect(releases?.map((release) => release.tag)).toEqual(['v0.2.0', 'v0.1.0']);
	});

	it('filters draft releases', async () => {
		const { fetcher } = makeFetcher([
			{ body: [githubRelease('v0.2.0'), githubRelease('v0.3.0', true)] }
		]);

		const releases = await getReleases(fetcher);

		expect(releases?.map((release) => release.tag)).toEqual(['v0.2.0']);
	});

	it('does not follow a next link outside api.github.com', async () => {
		const { fetcher, calls } = makeFetcher([
			{
				body: [githubRelease('v0.1.0')],
				headers: {
					Link: '<https://api.github.com.evil.example/releases?page=2>; rel="next"'
				}
			}
		]);

		const releases = await getReleases(fetcher, { all: true });

		expect(calls).toHaveLength(1);
		expect(releases?.map((release) => release.tag)).toEqual(['v0.1.0']);
	});

	it('returns collected releases if a later page fails', async () => {
		const { fetcher, calls } = makeFetcher([
			{ body: [githubRelease('v0.1.0')], headers: { Link: `<${nextPageUrl}>; rel="next"` } },
			{ body: {}, status: 503 }
		]);

		const releases = await getReleases(fetcher, { all: true });

		expect(calls).toHaveLength(2);
		expect(releases?.map((release) => release.tag)).toEqual(['v0.1.0']);
	});

	it('sends Authorization only when the token is set', async () => {
		const anonymous = makeFetcher([{ body: [] }]);
		await getReleases(anonymous.fetcher);
		expect(new Headers(anonymous.calls[0].init?.headers).get('Authorization')).toBeNull();

		mockEnv.githubToken = 't';
		const authenticated = makeFetcher([{ body: [] }]);
		await getReleases(authenticated.fetcher);
		expect(new Headers(authenticated.calls[0].init?.headers).get('Authorization')).toBe('Bearer t');
	});
});
