import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import devcert from 'devcert'

//https://vitejs.dev/config/
// export default defineConfig({
//   plugins: [svelte()],
//   server: {
//     host: true
//   }
// })

export default defineConfig(async () => {
  const { key, cert } = await devcert.certificateFor('localhost');
  //const { key, cert } = await devcert.certificateFor('192.168.1.3');

  return {
    root: './',
    plugins:  [svelte()],
    server: {
      open: true,
      https: {
        key,
        cert,
      },
      host: true
    },
  }
})
