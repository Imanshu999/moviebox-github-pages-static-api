# MovieBox Static Catalog API

यह repository मौजूदा MovieBox provider से metadata लेकर static JSON files बनाती है। JSON files GitHub Actions से generate और GitHub Pages पर publish होती हैं। कोई हमेशा चलने वाला API server आवश्यक नहीं है।

## Endpoints

- `GET /v1/manifest.json` — API version, generated timestamp और endpoint सूची
- `GET /v1/catalog.json` — पूरा उपलब्ध catalog
- `GET /v1/trending.json` — उपलब्ध browse ranking/metrics के अनुसार क्रम
- `GET /v1/latest.json` — उपलब्ध year के अनुसार क्रम
- `GET /v1/movies.json` — movie entries
- `GET /v1/series.json` — series entries
- `GET /v1/categories.json` — detail responses से उपलब्ध genres

## Android / Web integration

पहले `manifest.json` पढ़ें, फिर catalog JSON डाउनलोड करके app में local search/filter/sort करें। उदाहरण:

```javascript
const API_BASE_URL = "https://OWNER.github.io/REPOSITORY";
const response = await fetch(`${API_BASE_URL}/v1/catalog.json`);
if (!response.ok) throw new Error(`Catalog request failed: ${response.status}`);
const catalog = await response.json();
const results = catalog.results ?? [];
```

`OWNER` और `REPOSITORY` अपनी GitHub Pages project site के अनुसार बदलें। सही deployed base URL Actions run के Summary में मिलेगा।

## Publish

1. Repository **Settings → Pages → Build and deployment → Source** में **GitHub Actions** चुनें।
2. **Actions** tab → `Build and Publish MovieBox Static API` → **Run workflow**।
3. सफल deployment के बाद run Summary में endpoint links उपलब्ध होंगे।

किसी Render/Railway/Vercel account, API key, `API_BASE_URL` secret या variable की जरूरत नहीं है।

## सीमाएँ

- यह static hosting है; `/search?q=...` जैसा server-side dynamic search उपलब्ध नहीं है। Search/filter Android app में local JSON पर करें।
- Exporter मौजूदा MovieBox provider के homepage tab `2` और detail method का उपयोग करता है। Provider के बाहर की categories, pagination या metadata का दावा नहीं किया जाता।
- Trending और latest अलग upstream endpoints नहीं हैं: वे उपलब्ध browse metrics और year से locally sort किए गए catalog views हैं।
- Provider से अनुपलब्ध fields `null` या खाली list होंगे। कोई fake rating, poster, download link या video file नहीं बनाई जाती।
- Empty/invalid export पर workflow deployment रोक देता है; पिछला सफल Pages deployment बना रहता है।
