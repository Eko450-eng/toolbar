import { setTheme } from '$lib/stores/theme';

let options: string[] = [
	`Skeleton`,
	`modern`,
	`wintry`,
	`gold-nouveau`,
	`rocket`,
	`seafoam`,
	`sahara`,
	`crimson`,
	`vintage`,
	`hamlindigo`,
];

export function getThemeOptions(theme: string) {
	let results: any[] = [];

	options.forEach((option: any) => {
		results.push({
			text: option,
			enabled: theme !== option,
			action() {
				setTheme(option);
			},
		});
	});

	return results;
}
