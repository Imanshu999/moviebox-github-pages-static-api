# GitHub Actions से MovieBox Static API publish करें

## पहली बार setup

1. Project को अपनी GitHub repository की `main` या `master` branch में upload/push करें।
2. Repository → **Settings → Pages → Build and deployment → Source** में **GitHub Actions** चुनें।
3. कोई Render service, deploy hook, API key, secret या `API_BASE_URL` variable न बनाएँ।

## Manual publish

1. Repository का **Actions** tab खोलें।
2. `Build and Publish MovieBox Static API` workflow चुनें।
3. **Run workflow** दबाएँ।
4. Workflow Rust exporter compile करेगा, MovieBox provider से catalog metadata प्राप्त करेगा, JSON validate करेगा और GitHub Pages पर deploy करेगा।
5. सफल run के **Summary** में permanent Pages base URL और सभी endpoint links मिलेंगे।

## URL format

`https://<OWNER>.github.io/<REPOSITORY>/`

उदाहरण: owner `Imanshu999` और repository `MovieBox-Tui` होने पर:

`https://Imanshu999.github.io/MovieBox-Tui/v1/catalog.json`

अपनी repository का सही URL Actions Summary से लें।

## Automatic refresh

Workflow source files में relevant changes पर और हर 6 घंटे में scheduled refresh पर भी चलेगा। यदि provider empty/invalid catalog लौटाता है तो workflow fail होगा और deploy step नहीं चलेगा, जिससे पिछला सफल Pages deployment सुरक्षित रहेगा।

## Android app

`v1/manifest.json` और `v1/catalog.json` fetch करें। Static API server-side search नहीं देती; app में downloaded JSON पर local search, filter और sorting लागू करें। अधिक जानकारी: [API_README_HI.md](API_README_HI.md) और [STATIC_API_SETUP_HI.md](STATIC_API_SETUP_HI.md)।
