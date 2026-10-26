import { describe, expect, it } from 'vitest';
import { renderMarkdown } from './markdown';

describe('renderMarkdown', () => {
	it('renders GitHub markdown', () => {
		const html = renderMarkdown('## Fixes\n\n- **Tray** works\n\n[PR](https://github.com/x)');
		expect(html).toContain('<h3>Fixes</h3>');
		expect(html).toContain('<strong>Tray</strong>');
		expect(html).toContain('href="https://github.com/x"');
		expect(html).toContain('target="_blank"');
		expect(html).toContain('rel="noopener noreferrer"');
	});

	it('does not add new-tab attributes to relative or anchor links', () => {
		const html = renderMarkdown('[anchor](#x)\n\n[relative](/changelog)');
		expect(html).toContain('<a href="#x">anchor</a>');
		expect(html).toContain('<a href="/changelog">relative</a>');
		expect(html).not.toContain('target=');
		expect(html).not.toContain('rel=');
	});

	it('opens protocol-relative links in a new tab', () => {
		const html = renderMarkdown('[docs](//example.com)');
		expect(html).toContain('href="//example.com"');
		expect(html).toContain('target="_blank"');
		expect(html).toContain('rel="noopener noreferrer"');
	});

	it('does not pass through a user-supplied non-blank target', () => {
		const html = renderMarkdown('<a href="#x" target="_top">anchor</a>');
		expect(html).toContain('<a href="#x">anchor</a>');
		expect(html).not.toContain('target="_top"');
	});

	it('strips scripts, event handlers and javascript: links', () => {
		const html = renderMarkdown(
			'<script>alert(1)</script>\n\n<img src="x" onerror="alert(1)">\n\n[x](javascript:alert(1))'
		);
		expect(html).not.toContain('<script');
		expect(html).not.toContain('onerror');
		expect(html).not.toMatch(/href="javascript:/);
	});
});
