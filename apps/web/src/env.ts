import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	GITHUB_TOKEN: {
		description: 'Optional token to raise the GitHub API rate limit.',
		schema: (value) => value,
		static: false
	}
});
