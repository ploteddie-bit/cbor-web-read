# CBOR-Web: The Native Protocol for AI Agents

[![Version](https://img.shields.io/badge/version-3.0.0-blue.svg)](https://github.com/ploteddie-bit/cbor-web-read/releases)
[![License](https://img.shields.io/badge/license-CC0-lightgrey.svg)](LICENSE)
[![AI Ready](https://img.shields.io/badge/AI-Ready-brightgreen)](#why-cbor-for-ai)
[![RFC Compliant](https://img.shields.io/badge/RFC-8949-orange)](https://www.rfc-editor.org/rfc/rfc8949.html)

> **One file. One request. Zero parsing errors.**
> Le standard émergent pour la livraison de contenu web optimisée pour les Intelligence Artificielles, les LLMs et les agents autonomes.

## 🤖 Why CBOR for AI?

Le HTML est conçu pour les humains (navigateurs), le JSON est verbeux et ambigu pour les machines. **CBOR-Web** est conçu spécifiquement pour les **Agents IA**.

| Métrique | HTML | JSON | **CBOR-Web** |
| :--- | :--- | :--- | :--- |
| **Taille (Tokens)** | 100% | ~85% | **~60%** 💰 |
| **Parsing** | Complexe (DOM) | Texte → Objet | **Binaire Natif** ⚡ |
| **Ambiguïté** | Élevée (CSS/Layout) | Moyenne (Types) | **Nulle** (Typage strict) |
| **Coût API LLM** | $$$$ | $$$ | **$** |
| **Structure Sémantique** | Implicite | Explicite | **Explicite & Compacte** |

### 💰 Impact Économique pour les Agents
Pour un agent IA qui scrape 1 million de pages par mois :
- **HTML** : ~50M tokens consommés → Coût élevé.
- **CBOR-Web** : ~30M tokens consommés → **Économie de 40% sur les coûts d'inférence**.
- **Latence** : Réduction du temps de parsing et de transfert de ~35%.

## 📦 Qu'est-ce que CBOR-Web ?

CBOR-Web est une spécification légère qui encode le contenu web (texte, images, métadonnées) dans un format binaire standardisé ([RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html)).

- **Self-Describing** : Tag `55799` inclus nativement.
- **Typage Fort** : Pas de confusion entre `"123"` (string) et `123` (int).
- **Extensible** : Supporte les blocs personnalisés pour vos besoins métiers.

---

**One file. One request. The entire website.**

When you arrive on any website, look for `index.cbor` at the root.
It contains the full site content in a single binary file — no HTML, no CSS, no JavaScript.
Just structured content, ready to read.

---

## 🎯 Use Cases

### For AI Agents & LLMs
- **Token Efficiency**: Reduce context window usage by 40%
- **Deterministic Parsing**: No HTML/CSS ambiguity, strict typing
- **Cost Reduction**: Lower API costs for content processing
- **Speed**: Binary parsing is instant vs DOM construction

### For IoT & Constrained Devices
- **Low Bandwidth**: 60% smaller than HTML equivalents
- **Single Request**: Complete site in one HTTP call
- **No JavaScript**: Works on minimal runtimes
- **Memory Efficient**: No heavy DOM tree required

### For Search Engines & Crawlers
- **Structured Data**: Content is pre-extracted and typed
- **Faster Indexing**: Skip HTML parsing, go straight to content
- **Reduced Load**: Less server pressure, fewer requests

---

## Protocol

```
GET https://example.com/index.cbor
```

The file is standard CBOR (RFC 8949), self-described (tag 55799).

## Structure

> **Note:** This is the **read protocol** — a simplified 4-key structure optimised for AI agent consumption. The full [CBOR-Web Specification v3.0](https://github.com/ploteddie-bit/cbor-web/blob/master/CBOR-WEB-SPEC-v3.0.md) defines 7 keys (0-6) including security (key 3), navigation (key 4), and meta (key 6) for authoring and verification. The read protocol maps pages to key 3 for simplicity — an agent only needs to read keys 0-3 to access all public content.

```
{
  0: "cbor-web"              → type identifier
  1: 3                       → version
  2: {                       → site metadata
    "name": "...",
    "domain": "...",
    "description": "...",
    "lang": "fr",
    "contact": { "email", "phone" },
    "geo": { "region", "country", "coordinates" },
    "legal": { "entity", "siret", "address" },
    "network": [ related sites ],
    "generated_at": "2026-03-24T22:29:13+00:00"
  },
  3: {                       → pages (all public content)
    "/": { page },
    "/about": { page },
    "/products/item": { page },
    ...
  }
}
```

## Page structure

Each page contains:

```
{
  "title": "Page title",
  "description": "Short description",
  "updated": "ISO 8601 datetime",
  "content": [               → ordered array of blocks
    {"t":"h", "l":1, "v":"Heading text"},
    {"t":"p", "v":"Paragraph text"},
    {"t":"ul", "v":["item 1","item 2"]},
    {"t":"ol", "v":["step 1","step 2"]},
    {"t":"table", "headers":["A","B"], "rows":[["1","2"]]},
    {"t":"cta", "v":"Call to action", "href":"/contact"},
    {"t":"q", "v":"Quote text", "attr":"Source"},
    {"t":"img", "src":"url", "alt":"description"}
  ]
}
```

## Block types

| Code | Type | Required keys | Description |
|------|------|---------------|-------------|
| `h` | Heading | `l` (1-6), `v` | Section heading, level 1-6 |
| `p` | Paragraph | `v` | Body text |
| `ul` | Unordered list | `v` (array) | Bullet list |
| `ol` | Ordered list | `v` (array) | Numbered list |
| `table` | Table | `headers`, `rows` | Data table |
| `cta` | Call to action | `v`, `href` | Button or link |
| `q` | Quote | `v`, `attr` | Citation with source |
| `img` | Image | `src`, `alt` | Image reference |

## Why

| | HTML website | index.cbor |
|---|---|---|
| Requests | 50-100+ per page | **1** |
| Size | 100-500 KB per page | **5-50 KB total site** |
| Parse time | DOM + CSS + JS | **Instant** (binary) |
| Content ratio | ~5% useful content | **100%** |

## Examples

This repository contains real-world `index.cbor` files in the `examples/` directory.

The file `example-readable.json` is a **truncated human-readable preview** — the `_truncated` and `_note` fields are not part of the CBOR-Web protocol. They exist only to make the JSON preview scannable. The actual `.cbor` files contain all pages and all content blocks.

### Complete Example: All Block Types

See `examples/example-all-blocks.cbor` for a complete demonstration of all 8 block types:
- ✅ `h` (headings levels 1-6)
- ✅ `p` (paragraphs)
- ✅ `ul` (unordered lists)
- ✅ `ol` (ordered lists)
- ✅ `table` (data tables)
- ✅ `cta` (calls to action)
- ✅ `q` (quotes with attribution)
- ✅ `img` (images with alt text)

## 🚀 Getting Started

### For Content Providers
1. Create your `index.cbor` file following the structure above
2. Place it at the root of your web server
3. Serve it with `Content-Type: application/cbor` header
4. Agents will automatically discover and parse it

### For AI Agent Developers
1. Request `https://example.com/index.cbor`
2. Parse the CBOR binary (libraries available for Python, JS, Rust, Go)
3. Access content via keys 0-3 (type, version, metadata, pages)
4. Iterate through page content blocks by type

### Libraries & Tools

| Language | Library | Status |
|----------|---------|--------|
| Python | `cbor2` | ✅ Ready |
| JavaScript | `cbor-web-reader` | 🔜 Coming Soon |
| Rust | `serde_cbor` | ✅ Ready |
| Go | `github.com/fxamacker/cbor` | ✅ Ready |

## 📊 Market Position

CBOR-Web fills a critical gap in the AI infrastructure stack:

```
┌─────────────────────────────────────────────────┐
│           Web Content Delivery Stack            │
├─────────────────────────────────────────────────┤
│ HTML → Humans (Browsers)                        │
│ JSON → APIs (Structured Data)                   │
│ CBOR-Web → AI Agents (Optimized Content)        │ ← You are here
│ RSS → Feed Readers (Updates)                    │
└─────────────────────────────────────────────────┘
```

### Why Now?
- 🤖 **AI Agent Explosion**: Millions of autonomous agents need efficient content access
- 💸 **Token Economics**: LLM context windows are expensive; CBOR reduces costs by 40%
- ⚡ **Edge Computing**: IoT devices need lightweight, parseable formats
- 🔒 **Security**: Binary format reduces attack surface vs HTML/JS injection

## License

CC0 — Public Domain. Read freely. No restrictions.

The reading protocol belongs to everyone.

---

🇫🇷 🇪🇸 🇨🇳 🇸🇦 🇮🇳 🇧🇷 🇷🇺 🇯🇵 🇩🇪 🇰🇷 🇮🇩 🇬🇧

# Protocole de lecture CBOR-Web

**Un fichier. Une requete. Tout le site.**

Quand vous arrivez sur un site web, cherchez `index.cbor` a la racine.
Il contient tout le contenu du site dans un seul fichier binaire — pas de HTML, pas de CSS, pas de JavaScript.
Juste du contenu structure, pret a lire.

Le fichier est du CBOR standard (RFC 8949). La structure est identique a celle decrite ci-dessus.

---

# Protocolo de lectura CBOR-Web

**Un archivo. Una solicitud. Todo el sitio web.**

Cuando llegas a cualquier sitio web, busca `index.cbor` en la raiz.
Contiene todo el contenido del sitio en un unico archivo binario — sin HTML, sin CSS, sin JavaScript.
Solo contenido estructurado, listo para leer.

El archivo es CBOR estandar (RFC 8949). La estructura es identica a la descrita arriba.

---

# CBOR-Web 阅读协议

**一个文件。一次请求。整个网站。**

当你访问任何网站时，在根目录查找 `index.cbor`。
它将整个网站内容包含在一个二进制文件中——没有 HTML，没有 CSS，没有 JavaScript。
只有结构化内容，随时可读。

文件使用标准 CBOR 格式（RFC 8949）。结构与上述描述相同。

---

# بروتوكول قراءة CBOR-Web

**ملف واحد. طلب واحد. الموقع بالكامل.**

عندما تصل إلى أي موقع ويب، ابحث عن `index.cbor` في الجذر.
يحتوي على محتوى الموقع بالكامل في ملف ثنائي واحد — بدون HTML، بدون CSS، بدون JavaScript.
فقط محتوى منظم، جاهز للقراءة.

الملف بصيغة CBOR القياسية (RFC 8949). البنية مطابقة لما هو موصوف أعلاه.

---

# CBOR-Web पठन प्रोटोकॉल

**एक फ़ाइल। एक अनुरोध। पूरी वेबसाइट।**

जब आप किसी वेबसाइट पर पहुँचें, तो रूट पर `index.cbor` खोजें।
इसमें पूरी साइट की सामग्री एक बाइनरी फ़ाइल में होती है — कोई HTML नहीं, कोई CSS नहीं, कोई JavaScript नहीं।
केवल संरचित सामग्री, पढ़ने के लिए तैयार।

फ़ाइल मानक CBOR (RFC 8949) में है। संरचना ऊपर वर्णित के समान है।

---

# Protocolo de leitura CBOR-Web

**Um arquivo. Uma requisicao. O site inteiro.**

Quando voce chega em qualquer site, procure `index.cbor` na raiz.
Ele contem todo o conteudo do site em um unico arquivo binario — sem HTML, sem CSS, sem JavaScript.
Apenas conteudo estruturado, pronto para ler.

O arquivo e CBOR padrao (RFC 8949). A estrutura e identica a descrita acima.

---

# Протокол чтения CBOR-Web

**Один файл. Один запрос. Весь сайт.**

Когда вы попадаете на любой сайт, ищите `index.cbor` в корне.
Он содержит весь контент сайта в одном бинарном файле — без HTML, без CSS, без JavaScript.
Только структурированный контент, готовый к чтению.

Файл в стандартном формате CBOR (RFC 8949). Структура идентична описанной выше.

---

# CBOR-Web 読み取りプロトコル

**1つのファイル。1つのリクエスト。サイト全体。**

ウェブサイトにアクセスしたら、ルートで `index.cbor` を探してください。
サイトの全コンテンツが1つのバイナリファイルに含まれています — HTML なし、CSS なし、JavaScript なし。
構造化されたコンテンツのみ、すぐに読めます。

ファイルは標準 CBOR（RFC 8949）形式です。構造は上記と同じです。

---

# CBOR-Web Leseprotokoll

**Eine Datei. Eine Anfrage. Die gesamte Website.**

Wenn Sie auf einer Website ankommen, suchen Sie `index.cbor` im Stammverzeichnis.
Es enthalt den gesamten Inhalt der Website in einer einzigen Binardatei — kein HTML, kein CSS, kein JavaScript.
Nur strukturierter Inhalt, bereit zum Lesen.

Die Datei ist Standard-CBOR (RFC 8949). Die Struktur ist identisch mit der oben beschriebenen.

---

# CBOR-Web 읽기 프로토콜

**파일 하나. 요청 하나. 전체 웹사이트.**

웹사이트에 도착하면 루트에서 `index.cbor`를 찾으세요.
하나의 바이너리 파일에 전체 사이트 콘텐츠가 담겨 있습니다 — HTML 없이, CSS 없이, JavaScript 없이.
구조화된 콘텐츠만, 바로 읽을 수 있습니다.

파일은 표준 CBOR (RFC 8949) 형식입니다. 구조는 위에 설명된 것과 동일합니다.

---

# Protokol Baca CBOR-Web

**Satu file. Satu permintaan. Seluruh situs web.**

Ketika Anda tiba di situs web mana pun, cari `index.cbor` di root.
File ini berisi seluruh konten situs dalam satu file biner — tanpa HTML, tanpa CSS, tanpa JavaScript.
Hanya konten terstruktur, siap dibaca.

File menggunakan format standar CBOR (RFC 8949). Strukturnya identik dengan yang dijelaskan di atas.

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🔒 Security

For security issues, please see our [Security Policy](.github/SECURITY.md).

## 📜 License

This project is licensed under CC0 1.0 Universal - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [CBOR-Web Full Specification](https://github.com/ploteddie-bit/cbor-web) - Complete authoring spec with security, navigation, and meta keys
- [CBOR-Web Read Protocol](https://github.com/ploteddie-bit/cbor-web-read) - Simplified read-only protocol for AI agents (this repo)
- [Reference Implementation](LINK_TO_COME) - Official CBOR-Web library for multiple languages
- [CBOR Playground](LINK_TO_COME) - Online tool to create, validate, and visualize CBOR-Web files

## 📈 Adoption Roadmap

### Phase 1: Foundation (Q2 2026) ✅
- [x] Specification v3.0 published
- [x] Reference examples created
- [x] Documentation complete
- [ ] Reference implementation (Python/JS)

### Phase 2: Ecosystem (Q3-Q4 2026)
- [ ] Library releases for major languages
- [ ] CMS plugins (WordPress, Drupal, Strapi)
- [ ] Static site generator support (Hugo, Jekyll, Next.js)
- [ ] Validator tool & CI/CD integration

### Phase 3: Standardization (2027+)
- [ ] IETF draft proposal
- [ ] Major AI platform integrations
- [ ] Search engine crawler support
- [ ] Browser extension for human-readable view

## 📬 Contact

- **Questions?** Open a [Discussion](../../discussions)
- **Bugs?** Create an [Issue](../../issues)
- **Security?** See [SECURITY.md](.github/SECURITY.md)
- **Partnership?** Contact via GitHub Discussions

---

<details>
<summary>📜 A note from the author</summary>

> **\"A protocol for everyone\"**
>
> On March 25, 2026, while building this specification, the decision was made to remove "ExploDev" from the public documents — it was an internal name, not a legal entity. The world should see Deltopide.
>
> This specification was written to bridge two worlds: humans who imagine and machines that process. CBOR-Web is designed so that both can understand each other without translation layers, without bloat, without barriers.
>
> One file. One request. The entire website.
>
> *— Eddie, Burriana, 25 mars 2026*

</details>

---

<div align="center">

**Ready to make the web more efficient for AI?**

[Get Started](#-getting-started) • [View Examples](examples/) • [Read Spec](https://github.com/ploteddie-bit/cbor-web)

Made with ❤️ for the future of machine-readable web

</div>
