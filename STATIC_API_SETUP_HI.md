# MovieBox Static API — GitHub Pages Setup

यह export system मौजूदा MovieBox provider के homepage tab `2` और उसके वास्तविक detail responses का उपयोग करता है। यह कोई अलग server नहीं चलाता। GitHub Actions metadata को JSON में बदलकर GitHub Pages पर publish करता है।

## पहली बार setup

1. इस project को अपनी GitHub repository में upload/push करें। Repository का नाम और owner आपके अपने GitHub खाते के अनुसार होगा।
2. GitHub repository में **Settings → Pages → Build and deployment → Source** खोलें और **GitHub Actions** चुनें। किसी branch/folder source को चुनने की जरूरत नहीं है।
3. **Actions** tab खोलें। `Build and Publish MovieBox Static API` workflow चुनें और **Run workflow** दबाएँ।
4. Workflow पूरा होने के बाद run summary में वास्तविक Pages URL और सभी endpoint links मिलेंगे।

कोई Render/Railway/Vercel account, API key, API_BASE_URL variable या deploy secret आवश्यक नहीं है। Workflow `main`/`master` पर source changes और हर 6 घंटे के schedule पर भी चल सकता है।

## URL format

GitHub Pages project site का base URL:

`https://<OWNER>.github.io/<REPOSITORY>/`

उदाहरण के लिए, यदि owner `Imanshu999` और repository `MovieBox-Tui` है:

`https://Imanshu999.github.io/MovieBox-Tui/`

फिर endpoints:

- `v1/manifest.json`
- `v1/catalog.json`
- `v1/trending.json`
- `v1/latest.json`
- `v1/movies.json`
- `v1/series.json`
- `v1/categories.json`

Repository name बदलने पर URL में repository वाला हिस्सा भी बदलता है। सबसे सही base URL Actions deployment summary में दिया गया `page_url` है।

## Android app में उपयोग

`manifest.json` पहले पढ़ें और उसके `endpoints` से उपलब्ध JSON files लोड करें। सामान्य catalog endpoint `v1/catalog.json` है। App में search, sorting, genre filter और pagination/local filtering downloaded JSON पर करें; GitHub Pages server-side search API नहीं है। `id` provider item ID है और catalog में unique रखने के लिए provider prefix के बिना प्रकाशित किया गया है; `source` provider बताता है।

## Data limits और failure behavior

- Source data केवल existing MovieBox provider से आता है; metadata fields provider से उपलब्ध न हों तो `null` या खाली list रहेंगे।
- `trending.json` उपलब्ध browse ranking/metrics के आधार पर sort करता है; metric न होने पर उस entry का score शून्य माना जाता है। `latest.json` उपलब्ध year के आधार पर sort करता है। ये स्वतंत्र provider endpoints होने का दावा नहीं हैं।
- `categories.json` केवल detail responses में मिले genres से बनता है।
- Exporter empty catalog पर fail होता है। JSON validation fail होने पर Pages deployment शुरू नहीं होता, इसलिए पिछला सफल Pages deployment बना रहता है।
- यह project metadata export करता है, movie/video files, protected streams या download links publish नहीं करता।

## Local verification

Rust 1.90+ के साथ:

```bash
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo run --locked --release --bin moviebox-static-export -- dist
python3 -m json.tool dist/v1/catalog.json >/dev/null
```

Project documentation is also included under the deployed `/docs/` path so the Pages API root can coexist with the existing mdBook documentation.
