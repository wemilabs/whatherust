import { marked } from 'marked';
import sanitizeHtml from 'sanitize-html';

const allowedTags = [...sanitizeHtml.defaults.allowedTags, 'img', 'del', 'input'];

export function renderMarkdown(markdown: string): string {
	const html = marked.parse(markdown, { async: false, gfm: true });
	return sanitizeHtml(html, {
		allowedTags,
		allowedAttributes: {
			a: ['href', 'title', 'rel', 'target'],
			img: ['src', 'alt', 'title', 'width', 'height'],
			input: ['type', 'checked', 'disabled'],
			code: ['class']
		},
		allowedSchemes: ['http', 'https', 'mailto'],
		transformTags: {
			a: (tagName, attribs) => {
				const transformed = { ...attribs };
				if (/^https?:\/\//i.test(transformed.href ?? '')) {
					transformed.target = '_blank';
					transformed.rel = 'noopener noreferrer';
				} else if (transformed.target !== '_blank') {
					delete transformed.target;
				}
				return { tagName, attribs: transformed };
			},
			img: sanitizeHtml.simpleTransform('img', { loading: 'lazy' }),
			h1: 'h3',
			h2: 'h3'
		}
	});
}
