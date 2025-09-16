import eslint from '@eslint/js'
import tseslint from 'typescript-eslint'
import globals from 'globals'
import vue from 'eslint-plugin-vue'

export default tseslint.config(
  {ignores: ['dist', '.eslintrc.cjs', '**/.astro/**']},
  // Base ESLint recommended rules
  eslint.configs.recommended,

  // TypeScript recommended rules
  ...tseslint.configs.recommended,

  // Vue recommended rules
  ...vue.configs['flat/recommended'],

  {
    files: ['**/*.ts', '**/*.cts', '**/*.mts', '**/*.tsx'],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
      },
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2020,
      }
    },
    plugins: {
      '@typescript-eslint': tseslint.plugin,
    },
    rules: {
      '@typescript-eslint/no-unused-vars': 'off',
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      "@typescript-eslint/no-empty-object-type": "off",
      '@typescript-eslint/no-explicit-any': 'off',
      'no-unused-vars': 'off', // Use TypeScript version instead
      'no-undef': 'off', // TypeScript handles this
    }
  },

  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vue.parser,
      parserOptions: {
        ecmaVersion: 2020,
        sourceType: 'module',
        parser: '@typescript-eslint/parser',
      },
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2020,
      }
    },
    plugins: {
      vue: vue,
      '@typescript-eslint': tseslint.plugin,
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-unused-vars': 'warn',
      '@typescript-eslint/no-unused-vars': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      'no-undef': 'off', // TypeScript handles this
      'vue/max-attributes-per-line': 'off',
      'vue/singleline-html-element-content-newline': 'off',
      'vue/html-self-closing': 'off',
      'vue/attributes-order': 'off',
      "vue/no-dupe-v-else-if": "off"
    }
  }
)