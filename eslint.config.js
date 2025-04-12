import prettier from 'eslint-config-prettier'
import js from '@eslint/js'
import { includeIgnoreFile } from '@eslint/compat'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'
import { fileURLToPath } from 'node:url'
import ts from 'typescript-eslint'
import svelteConfig from './svelte.config.js'

const gitignorePath = fileURLToPath(new URL('./.gitignore', import.meta.url))

export default ts.config(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs.recommended,
	prettier,
	...svelte.configs.prettier,
	{
		languageOptions: {
			globals: { ...globals.browser, ...globals.node },
		},
		rules: { 'no-undef': 'off' },
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
		ignores: ['eslint.config.js', 'svelte.config.js'],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser,
				svelteConfig,
			},
		},
	},
	{
		rules: {
			// '@typescript-eslint/naming-convention': [
			// 	'error',
			// 	{
			// 		selector: 'variableLike',
			// 		format: ['snake_case', 'UPPER_CASE', 'PascalCase'],
			// 		leadingUnderscore: 'allow',
			// 	},
			// 	{
			// 		selector: 'parameter',
			// 		modifiers: ['destructured'],
			// 		format: null,
			// 	},
			// 	{
			// 		selector: 'variable',
			// 		modifiers: ['destructured'],
			// 		format: null,
			// 	},
			// ],
			'@typescript-eslint/no-unused-vars': [
				'error',
				{
					caughtErrorsIgnorePattern: '^_',
					argsIgnorePattern: '^_',
					varsIgnorePattern: '^_',
				},
			],
			eqeqeq: ['error', 'always'],
			'svelte/button-has-type': 'error',
			'svelte/require-each-key': 'off', // Unnecessary each key probably has performance downside
		},
	},
)
