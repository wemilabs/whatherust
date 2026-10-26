// Measured on the same Ubuntu 22.04.5 x86_64 VM; method in the "How we measured" footnote.
export const measuredOn = '2026-10-26';

export const measurements = {
	whatherust: { name: 'WhatheRust 0.1.0', deb: 7_591_030, appimage: 85_596_664, rssMiB: 1037.2 },
	electron: { name: 'Electron 45.0.0', deb: 101_044_140, appimage: 125_653_238, rssMiB: 868.8 }
} as const;

export function formatMB(bytes: number): string {
	return `${(bytes / 1_000_000).toFixed(1)} MB`;
}

export function formatMiB(mib: number): string {
	return `${Math.round(mib)} MiB`;
}
