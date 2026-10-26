import type { Release } from '#lib/types.js';

export type Platform = 'windows' | 'macos' | 'linux' | 'unknown';

type NavigatorInfo = {
	userAgentData?: { platform?: string; mobile?: boolean };
	userAgent: string;
};

type Asset = Release['assets'][number];

export function detectPlatform(nav: NavigatorInfo): Platform {
	if (nav.userAgentData?.mobile) return 'unknown';

	const platform = nav.userAgentData?.platform || nav.userAgent;
	if (/android|iphone|ipad|ipod|mobile|cros|chrome\s?os/i.test(platform)) return 'unknown';
	if (/windows/i.test(platform)) return 'windows';
	if (/mac|macintosh/i.test(platform)) return 'macos';
	if (/linux/i.test(platform)) return 'linux';
	return 'unknown';
}

function findAsset(assets: Asset[], pattern: RegExp): Asset | undefined {
	return assets.find((asset) => pattern.test(asset.name));
}

export function pickAssets(release: Pick<Release, 'assets'> | null): {
	windows: { setup?: Asset; msi?: Asset };
	macos: { dmg?: Asset };
	linux: { appimage?: Asset; deb?: Asset; rpm?: Asset };
} {
	const assets = release?.assets ?? [];

	return {
		windows: {
			setup: findAsset(assets, /-setup\.exe$/i),
			msi: findAsset(assets, /\.msi$/i)
		},
		macos: {
			dmg: findAsset(assets, /_universal\.dmg$/i)
		},
		linux: {
			appimage: findAsset(assets, /\.appimage$/i),
			deb: findAsset(assets, /\.deb$/i),
			rpm: findAsset(assets, /\.rpm$/i)
		}
	};
}

export function pickPrimary(
	platform: Platform,
	release: Pick<Release, 'assets'> | null
): { asset: Asset; kind: 'setup' | 'msi' | 'dmg' | 'appimage' | 'deb' | 'rpm' } | undefined {
	const assets = pickAssets(release);

	if (platform === 'windows') {
		if (assets.windows.setup) return { asset: assets.windows.setup, kind: 'setup' };
		if (assets.windows.msi) return { asset: assets.windows.msi, kind: 'msi' };
	}

	if (platform === 'macos' && assets.macos.dmg) {
		return { asset: assets.macos.dmg, kind: 'dmg' };
	}

	if (platform === 'linux') {
		if (assets.linux.appimage) return { asset: assets.linux.appimage, kind: 'appimage' };
		if (assets.linux.deb) return { asset: assets.linux.deb, kind: 'deb' };
		if (assets.linux.rpm) return { asset: assets.linux.rpm, kind: 'rpm' };
	}

	return undefined;
}
