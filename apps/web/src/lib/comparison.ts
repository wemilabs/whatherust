export const measurements = {
	whatherust: { name: 'WhatheRust 0.1.0', deb: 7_591_030, appimage: 85_596_664 },
	electron: { name: 'Electron 45.0.0', deb: 101_044_140, appimage: 125_653_238 }
} as const;

export const macosMeasurements = {
	whatherust: {
		name: 'WhatheRust 0.1.0',
		dmgArm64: 6_260_496,
		installedApp: 18_112_512
	},
	electron: {
		name: 'Electron 45.0.0',
		dmgArm64: 120_489_258,
		installedApp: 270_184_448
	}
} as const;

export const windowsMeasurements = {
	whatherust: {
		name: 'WhatheRust 0.1.0',
		installer: 3_752_083,
		installedApp: 15_771_551
	},
	electron: {
		name: 'Electron 45.0.0',
		installer: 121_197_184,
		installedApp: 374_236_068
	}
} as const;

export function formatMB(bytes: number): string {
	return `${(bytes / 1_000_000).toFixed(1)} MB`;
}
