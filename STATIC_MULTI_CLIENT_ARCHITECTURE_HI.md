# Static multi-client architecture

This repository keeps the existing terminal application and its Rust provider/core code. The same provider service generates a static JSON snapshot for GitHub Pages. The website, Android client and third-party integrations consume that published contract.

## No always-on server

The default deployment uses GitHub Actions to run the existing Rust static exporter every six hours and on source changes/manual dispatch. GitHub Pages serves the generated JSON and static web clients. This avoids a paid VM, database, container host or continuously running API process.

- API base: https://imanshu999.github.io/moviebox-github-pages-static-api/v1/
- Web client: https://imanshu999.github.io/moviebox-github-pages-static-api/web/
- API Control & Tester: https://imanshu999.github.io/moviebox-github-pages-static-api/tester/
- Integration Playground: https://imanshu999.github.io/moviebox-github-pages-static-api/playground/

## Current endpoints

- `manifest.json`
- `catalog.json`
- `trending.json`
- `latest.json`
- `movies.json`
- `series.json`
- `categories.json`

Catalog search/filter is client-side. Clients should cache the last successful JSON response and show its generatedAt timestamp. If a scheduled refresh fails, the last successful Pages deployment remains online because the workflow refuses to publish an empty catalog.

## Client integration

Website JavaScript:

```js
const API = "https://imanshu999.github.io/moviebox-github-pages-static-api/v1/";
const response = await fetch(API + "catalog.json");
if (!response.ok) throw new Error("Catalog request failed");
const catalog = await response.json();
const titles = catalog.results;
```

Android: request the same `catalog.json` URL using OkHttp/Ktor and map `results` into the app's model. The Integration Playground contains a Kotlin starting example.

## Honest feature boundary

The static exporter currently guarantees catalog metadata. It does not yet export the complete detail tree, season/episode resources, audio-track choices, subtitle files, or playable stream manifests. The Rust service already has richer provider methods and a separate Axum API binary, but those must be deliberately exposed and tested before clients can promise those features. A catalog poster or detail URL is not a playable video URL.

GitHub Pages cannot execute the Axum server. The existing `moviebox-api` binary is retained for optional self-hosting, but the default Pages setup does not run or pay for a server. Playback must use provider-authorized playable URLs and a platform-compatible player; this project does not host media.

## Pages deployment

Enable GitHub Pages with **GitHub Actions** as the source in repository Settings → Pages. The workflow deploys the JSON snapshot and all three client pages. Open the Actions tab and check the latest “Publish MovieBox Static API and Web Clients” run.
