import { marked } from 'marked';
import sanitizeHtml from 'sanitize-html';

const allowedTags = [...sanitizeHtml.defaults.allowedTags, 'img', 'del', 'input'];

export function renderMarkdown(markdown: string): string {
	const html = marked.parse(markdown, { async: false, gfm: true });
	return sanitizeHtml(html, {
		allowedTags,
		allowedAttributes: {
			a: ['href', 'title', 'rel'],
			img: ['src', 'alt', 'title', 'width', 'height'],
			input: ['type', 'checked', 'disabled'],
			code: ['class']
		},
		allowedSchemes: ['http', 'https', 'mailto'],
		transformTags: {
			a: sanitizeHtml.simpleTransform('a', { rel: 'noopener noreferrer nofollow' }),
			img: sanitizeHtml.simpleTransform('img', { loading: 'lazy' }),
			h1: 'h3',
			h2: 'h3'
		}
	});
}
