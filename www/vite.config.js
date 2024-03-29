import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
    plugins: [
        topLevelAwait(),
        wasm(),
        VitePWA({
            registerType: 'autoUpdate',
            manifest: {
                name: "WASM bruteforce thingymajig",
                start_url: "https://brute.sly.ee",
                short_name: "WASM Bruteforce",
                description: "Bruteforce images from image hosting website",
                icons: [
                    {
                        src: "assets/android-chrome-192x192.png",
                        sizes: "192x192",
                        type: "image/png"
                    },
                    {
                        src: "assets/android-chrome-512x512.png",
                        sizes: "512x512",
                        type: "image/png"
                    }
                ],
                theme_color: "#1f1f1f",
                background_color: "#0f0f0f",
                display: "standalone"
            }
        }),
    ],
});