# MovieBox API Tester

यह read-only testing website है। यह static API के साथ GitHub Pages पर publish होती है।

## Website URL
`https://<OWNER>.github.io/<REPOSITORY>/tester/`

## Checks
- सात JSON endpoints का HTTP status और response time
- JSON parsing और catalog item count
- Duplicate IDs, missing title/name और missing poster count
- वास्तविक catalog का search और movie/series filter
- Endpoint response का JSON viewer

## Publish
1. Repository Settings → Pages → Build and deployment → Source में **GitHub Actions** चुनें।
2. Actions tab खोलें।
3. **Publish MovieBox API Tester** workflow चुनें और **Run workflow** दबाएँ।
4. सफल deployment के बाद ऊपर दिए गए URL पर tester खोलें।

यह workflow मौजूदा Rust exporter से JSON दोबारा बनाता है, फिर `tester/` वेबसाइट को उसी Pages deployment में जोड़ता है। किसी external server या API key की जरूरत नहीं है।
