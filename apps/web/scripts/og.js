// Renders the 1200x630 social preview image for each locale into static/og/.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { Resvg } from '@resvg/resvg-js';
import satori from 'satori';

const root = new URL('../', import.meta.url);
const read = (path) => readFileSync(new URL(path, root));
const font = (weight) => ({
	name: 'Geist',
	weight,
	style: 'normal',
	data: read(`node_modules/@fontsource/geist/files/geist-latin-${weight}-normal.woff`)
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
			background: '#f7f7f7',
			color: '#191919',
			fontFamily: 'Geist'
		},
		[
			{ type: 'img', props: { src: logo, width: 300, height: 300 } },
			h('div', { display: 'flex', flexDirection: 'column', width: 676 }, [
				h(
					'div',
					{ fontSize: 76, fontWeight: 400, letterSpacing: -2, color: '#8a8a8a' },
					'WhatheRust'
				),
				h(
					'div',
					{ fontSize: 38, lineHeight: 1.15, marginTop: 20, color: '#8a8a8a' },
					messages.ogTitleLead
				),
				h('div', { fontSize: 38, lineHeight: 1.15, color: '#191919' }, messages.ogTitle),
				h(
					'div',
					{ fontSize: 26, marginTop: 36, color: '#c2410c', fontWeight: 800 },
					'Rust + Tauri · Windows · macOS · Linux'
				)
			])
		]
	);
	const svg = await satori(tree, { width: 1200, height: 630, fonts });
	const png = new Resvg(svg, { fitTo: { mode: 'width', value: 1200 } }).render().asPng();
	writeFileSync(new URL(`static/og/${locale}.png`, root), png);
}
