import { defineConfig } from 'vite';

export default defineConfig({
    root: 'web',
    build: {
        outDir: '../dist',
        emptyOutDir: true,
        assetsInlineLimit: 0,
        minify: 'terser',
        cssMinify: 'lightningcss',
        cssCodeSplit: true,
        terserOptions: {
            compress: {
                drop_console: true,
                drop_debugger: true,
            },
        },
        rollupOptions: {
            output: {
                entryFileNames: '[name].[hash].js',
                chunkFileNames: '[name].[hash].js',
                assetFileNames: '[name].[hash].[ext]',
            },
        },
    },
    css: {
        transformer: 'lightningcss',
        lightningcss: {
            targets: {
                chrome: 95,
                firefox: 95,
                safari: 14,
            },
            minify: true,
        },
    },
});