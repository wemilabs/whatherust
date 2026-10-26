export type Release = {
	tag: string;
	name: string;
	url: string;
	publishedAt: string | null;
	prerelease: boolean;
	body: string;
	assets: { name: string; url: string; size: number }[];
};
