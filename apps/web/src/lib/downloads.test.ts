import { describe, expect, it } from 'vitest';
import { detectPlatform, pickAssets, pickPrimary } from './downloads';
import type { Release } from './types';

const release: Release = {
	tag: 'v0.1.0',
	name: 'WhatheRust 0.1.0',
	url: 'https://github.com/wemilabs/whatherust/releases/tag/v0.1.0',
	publishedAt: null,
	prerelease: false,
	body: '',
	assets: [
		{
			name: 'WhatheRust_0.1.0_x64-setup.exe',
			url: 'https://example.com/setup.exe',
			size: 100
		},
		{
			name: 'WhatheRust_0.1.0_x64_en-US.msi',
			url: 'https://example.com/app.msi',
			size: 101
		},
		{
			name: 'WhatheRust_0.1.0_universal.dmg',
			url: 'https://example.com/app.dmg',
			size: 102
		},
		{
			name: 'WhatheRust_0.1.0_amd64.AppImage',
			url: 'https://example.com/app.AppImage',
			size: 103
		},
		{
			name: 'WhatheRust_0.1.0_amd64.AppImage.sig',
			url: 'https://example.com/app.AppImage.sig',
			size: 104
		},
		{
			name: 'WhatheRust_0.1.0_amd64.deb',
			url: 'https://example.com/app.deb',
			size: 105
		},
		{
			name: 'WhatheRust-0.1.0-1.x86_64.rpm',
			url: 'https://example.com/app.rpm',
			size: 106
		},
		{
			name: 'WhatheRust_universal.app.tar.gz',
			url: 'https://example.com/app.tar.gz',
			size: 107
		},
		{
			name: 'latest.json',
			url: 'https://example.com/latest.json',
			size: 108
		}
	]
};

describe('detectPlatform', () => {
	it.each([
		['Windows NT 10.0; Win64; x64', 'windows'],
		['Macintosh; Intel Mac OS X 10_15_7', 'macos'],
		['X11; Ubuntu; Linux x86_64', 'linux'],
		['Linux; Android 14', 'unknown'],
		['iPhone; CPU iPhone OS 17_0 like Mac OS X', 'unknown'],
		['iPad; CPU OS 17_0 like Mac OS X', 'unknown'],
		['X11; CrOS x86_64 14541.0.0', 'unknown'],
		['Mozilla/5.0 (Linux; Android 14) Mobile', 'unknown']
	] as const)('detects %s as %s', (userAgent, expected) => {
		expect(detectPlatform({ userAgent })).toBe(expected);
	});

	it('uses userAgentData.platform when present', () => {
		expect(
			detectPlatform({
				userAgentData: { platform: 'Windows', mobile: false },
				userAgent: 'Macintosh'
			})
		).toBe('windows');
	});

	it('treats userAgentData mobile devices as unknown', () => {
		expect(
			detectPlatform({
				userAgentData: { platform: 'Android', mobile: true },
				userAgent: 'Windows'
			})
		).toBe('unknown');
	});

	it('accepts an iPad in desktop mode as macOS', () => {
		expect(detectPlatform({ userAgent: 'Macintosh; Intel Mac OS X 10_15_7' })).toBe('macos');
	});

	it('returns unknown for unrecognized platforms', () => {
		expect(detectPlatform({ userAgent: 'Some new platform' })).toBe('unknown');
	});
});

describe('pickAssets', () => {
	it('selects installable assets and excludes signatures and archives', () => {
		const selected = pickAssets(release);

		expect(selected.windows.setup).toEqual(release.assets[0]);
		expect(selected.windows.msi).toEqual(release.assets[1]);
		expect(selected.macos.dmg).toEqual(release.assets[2]);
		expect(selected.linux.appimage).toEqual(release.assets[3]);
		expect(selected.linux.deb).toEqual(release.assets[5]);
		expect(selected.linux.rpm).toEqual(release.assets[6]);
	});

	it('returns empty platform groups for a null release', () => {
		expect(pickAssets(null)).toEqual({
			windows: { setup: undefined, msi: undefined },
			macos: { dmg: undefined },
			linux: { appimage: undefined, deb: undefined, rpm: undefined }
		});
	});

	it('returns empty platform groups when no assets are available', () => {
		expect(pickAssets({ ...release, assets: [] })).toEqual({
			windows: { setup: undefined, msi: undefined },
			macos: { dmg: undefined },
			linux: { appimage: undefined, deb: undefined, rpm: undefined }
		});
	});

	it('ignores stable-named installer copies', () => {
		expect(
			pickAssets({
				assets: [
					{
						name: 'WhatheRust-windows-x64-setup.exe',
						url: 'https://example.com/stable-windows.exe',
						size: 100
					},
					{
						name: 'WhatheRust-macos-universal.dmg',
						url: 'https://example.com/stable-macos.dmg',
						size: 101
					},
					{
						name: 'WhatheRust-linux-x86_64.AppImage',
						url: 'https://example.com/stable-linux.AppImage',
						size: 102
					}
				]
			})
		).toEqual({
			windows: { setup: undefined, msi: undefined },
			macos: { dmg: undefined },
			linux: { appimage: undefined, deb: undefined, rpm: undefined }
		});
	});
});

describe('pickPrimary', () => {
	it.each([
		['windows', 0, 'setup'],
		['macos', 2, 'dmg'],
		['linux', 3, 'appimage']
	] as const)('selects the primary %s asset', (platform, assetIndex, kind) => {
		expect(pickPrimary(platform, release)).toEqual({ asset: release.assets[assetIndex], kind });
	});

	it('returns undefined for unknown platforms', () => {
		expect(pickPrimary('unknown', release)).toBeUndefined();
	});

	it('returns undefined when the platform primary asset is missing', () => {
		expect(pickPrimary('windows', { assets: [release.assets[2]] })).toBeUndefined();
	});

	it('returns undefined for a null release', () => {
		expect(pickPrimary('linux', null)).toBeUndefined();
	});

	it('falls back to MSI when the Windows setup asset is missing', () => {
		expect(pickPrimary('windows', { assets: [release.assets[1]] })).toEqual({
			asset: release.assets[1],
			kind: 'msi'
		});
	});

	it('selects DEB before RPM when AppImage is missing', () => {
		expect(pickPrimary('linux', { assets: [release.assets[5], release.assets[6]] })).toEqual({
			asset: release.assets[5],
			kind: 'deb'
		});
	});

	it('falls back to RPM when AppImage and DEB are missing', () => {
		expect(pickPrimary('linux', { assets: [release.assets[6]] })).toEqual({
			asset: release.assets[6],
			kind: 'rpm'
		});
	});
});
