import { defineConfig } from 'vite';

export default defineConfig({
    root: 'web',
    build: {
        outDir: '../dist',
        emptyOutDir: true,
        assetsInlineLimit: 0,
        rollupOptions: {
            output: {
                entryFileNames: '[name].[hash].js',
                chunkFileNames: '[name].[hash].js',
                assetFileNames: '[name].[hash].[ext]',
            },
        },
    },
});