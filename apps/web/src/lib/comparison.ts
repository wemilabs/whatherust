// Measured on the same Ubuntu 22.04.5 x86_64 VM; method in the "How we measured" footnote.
export const measuredOn = '2026-10-26';

export const measurements = {
	whatherust: { name: 'WhatheRust 0.1.0', deb: 7_591_030, appimage: 85_596_664, rssMiB: 1037.2 },
	electron: { name: 'Electron 45.0.0', deb: 101_044_140, appimage: 125_653_238, rssMiB: 868.8 }
} as const;

export const macosMeasurements = {
	whatherust: {
		name: 'WhatheRust 0.1.0',
		dmgArm64: 6_260_496,
		installedApp: 18_112_512,
		rssMedianMiB: 747.22,
		rssRunsMiB: [666.98, 747.22]
	},
	electron: {
		name: 'Electron 45.0.0',
		dmgArm64: 120_489_258,
		installedApp: 270_184_448,
		rssMedianMiB: 877.39,
		rssRunsMiB: [869.61, 877.39]
	}
} as const;

export const windowsMeasurements = {
	whatherust: {
		name: 'WhatheRust 0.1.0',
		installer: 3_752_083,
		installedApp: 15_771_551,
		workingSetMedianMiB: 559.39,
		workingSetRunsMiB: [549.36, 559.39]
	},
	electron: {
		name: 'Electron 45.0.0',
		installer: 121_197_184,
		installedApp: 374_236_068,
		workingSetMedianMiB: 497.21,
		workingSetRunsMiB: [483.77, 497.21]
	}
} as const;

export function formatMB(bytes: number): string {
	return `${(bytes / 1_000_000).toFixed(1)} MB`;
}

export function formatMiB(mib: number): string {
	return `${Math.round(mib)} MiB`;
}
