// Renders the 1200x630 social preview image for each locale into static/og/.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { Resvg } from '@resvg/resvg-js';
import satori from 'satori';

const root = new URL('../', import.meta.url);
const read = (path) => readFileSync(new URL(path, root));
const font = (weight) => ({
	name: 'Inter',
	weight,
	style: 'normal',
	data: read(`node_modules/@fontsource/inter/files/inter-latin-${weight}-normal.woff`)
});
const fonts = [font(400), font(800)];
const logo = `data:image/svg+xml;base64,${read('src/lib/assets/logo.svg').toString('base64')}`;
const { locales } = JSON.parse(read('project.inlang/settings.json'));

const h = (type, style, children) => ({ type, props: { style, children } });

mkdirSync(new URL('static/og/', root), { recursive: true });

for (const locale of locales) {
	const messages = JSON.parse(read(`messages/${locale}.json`));
	const tree = h(
		'div',
		{
			width: '100%',
			height: '100%',
			display: 'flex',
			alignItems: 'center',
			gap: 64,
			padding: 80,
			background: 'linear-gradient(135deg, #1c1917 0%, #0c0a09 60%, #2a1208 100%)',
			color: '#fafaf9',
			fontFamily: 'Inter'
		},
		[
			{ type: 'img', props: { src: logo, width: 300, height: 300 } },
			h('div', { display: 'flex', flexDirection: 'column', flex: 1 }, [
				h('div', { fontSize: 76, fontWeight: 800, letterSpacing: -2 }, 'WhatheRust'),
				h(
					'div',
					{ fontSize: 38, lineHeight: 1.25, marginTop: 20, color: '#e7e5e4' },
					messages.heroTitle
				),
				h(
					'div',
					{ fontSize: 26, marginTop: 36, color: '#25d366', fontWeight: 800 },
					'Rust + Tauri · Windows · macOS · Linux'
				)
			])
		]
	);
	const svg = await satori(tree, { width: 1200, height: 630, fonts });
	const png = new Resvg(svg, { fitTo: { mode: 'width', value: 1200 } }).render().asPng();
	writeFileSync(new URL(`static/og/${locale}.png`, root), png);
}
