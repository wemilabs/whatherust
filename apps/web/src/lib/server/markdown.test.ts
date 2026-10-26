import { describe, expect, it } from 'vitest';
import { renderMarkdown } from './markdown';

describe('renderMarkdown', () => {
	it('renders GitHub markdown', () => {
		const html = renderMarkdown('## Fixes\n\n- **Tray** works\n\n[PR](https://github.com/x)');
		expect(html).toContain('<h3>Fixes</h3>');
		expect(html).toContain('<strong>Tray</strong>');
		expect(html).toContain('href="https://github.com/x"');
		expect(html).toContain('rel="noopener noreferrer nofollow"');
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
