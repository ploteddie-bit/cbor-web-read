# CBOR-Web — The Story

> Select your language / Choisissez votre langue / Seleccione su idioma

[English](#english) · [Francais](#francais) · [Espanol](#espanol) · [中文](#中文) · [العربية](#العربية) · [हिन्दी](#हिन्दी) · [Portugues](#portugues) · [Русский](#русский) · [日本語](#日本語) · [Deutsch](#deutsch) · [한국어](#한국어) · [Bahasa Indonesia](#bahasa-indonesia)

---

<a id="english"></a>
## 🇬🇧 English

### Chapter 1 — Two researchers, one idea (2013)

In 2013, two researchers — Carsten Bormann from the University of Bremen and Paul Hoffman from ICANN — faced a concrete problem: connected objects (IoT sensors, smart cards, industrial controllers) needed to exchange structured data, but JSON was too heavy. Too verbose. Too slow to parse on a processor with 8 KB of memory.

They invented **CBOR** — Concise Binary Object Representation. The same data model as JSON, but encoded in binary. Compact, fast, extensible. Published as RFC 7049 by the IETF in October 2013.

CBOR wasn't designed for the web. It was designed for constrained machines. Thermometers, door locks, electricity meters. Things that count every byte.

### Chapter 2 — Maturity and standardisation (2020)

Seven years later, CBOR had proven itself. Adopted by CoAP (the HTTP of IoT), integrated into WebAuthn (FIDO2 authentication keys), used by COSE for cryptographic signatures. Millions of devices spoke CBOR without anyone noticing.

In December 2020, RFC 8949 replaced the original specification. Clarifications, corrections, deterministic encoding. CBOR became an Internet Standard — the highest level of validation at the IETF.

But the web continued to ignore it. Websites kept sending megabytes of HTML, CSS, and JavaScript to display a few paragraphs of text.

### Chapter 3 — The turning point: AI agents arrive (2024-2026)

Then everything changed. AI agents started browsing the web. Not with eyes and a mouse — with HTTP requests and parsers. And what they found was absurd:

- **2.86 MB** average per page in 2025 (HTTP Archive)
- **86 requests** to display a single page
- **95% noise** (CSS, JavaScript, tracking, ads) for **5% useful content**

An AI agent that wants to read a restaurant menu must download 3 MB of React code, execute JavaScript, wait for 47 API calls, fight through cookie banners, and parse a DOM of 2,000 nodes. To find: address, phone number, opening hours, menu. 200 bytes of useful information buried under megabytes of decoration.

### Chapter 4 — CBOR-Web: one file, the entire site (2026)

The idea was obvious, yet no one had done it: **what if an entire website could be a single binary file at the root?**

`index.cbor` — placed next to `index.html`. Same address, different format. The HTML for humans, the CBOR for machines.

One file. One request. Zero JavaScript. Zero CSS. Just structured content: headings, paragraphs, lists, tables, links, calls to action. Everything an AI agent needs, nothing it doesn't.

A 160 KB Shopify page becomes 8 KB of CBOR. A complete 84-page site fits in 700 KB. Fifteen times lighter. One request instead of eighty-six.

### Chapter 5 — The ecological imperative

The numbers are staggering:

- Data centres consumed **460 TWh** of electricity in 2022 — projected to reach **620-1,050 TWh** by 2026
- The digital sector's energy consumption grows **9% per year**
- An average website with 10,000 monthly visits emits **31.5 kg of CO2** per year
- Web page weight has grown **+203%** since 2015

Every unnecessary byte transferred is energy wasted. Every CSS animation that no machine will ever see is a server running, a cable carrying data, a router routing packets — for nothing.

CBOR-Web doesn't just make the web faster for AI. It makes it **lighter for the planet**.

| | Traditional HTML | CBOR-Web |
|---|---|---|
| Average page weight | 2.86 MB | 5-50 KB |
| Requests per page | 50-100 | 1 |
| Useful content ratio | ~5% | 100% |
| Energy per read | High | Minimal |

If every website offered an `index.cbor` alongside its `index.html`, the volume of data transferred by AI agents would drop by a factor of **10 to 50**. Not through compression. Through elimination of the unnecessary.

### Chapter 6 — A standard that belongs to everyone

The CBOR-Web read protocol is published under **CC0 — Public Domain**. No licence to pay, no permission to ask, no patent to fear. Anyone can read an `index.cbor`. The reading protocol belongs to humanity.

What creates value is not reading — it is **creating**. Transforming an HTML website into a clean, structured, optimised `index.cbor` requires intelligence, tools, and expertise. That is where the publishers come in. That is where the ecosystem grows.

Like PDF: everyone can read. The tools that create are where the value lives.

---

<a id="francais"></a>
## 🇫🇷 Francais

### Chapitre 1 — Deux chercheurs, une idee (2013)

En 2013, deux chercheurs — Carsten Bormann de l'Universite de Breme et Paul Hoffman de l'ICANN — faisaient face a un probleme concret : les objets connectes (capteurs IoT, cartes a puce, controleurs industriels) devaient echanger des donnees structurees, mais le JSON etait trop lourd. Trop verbeux. Trop lent a analyser sur un processeur de 8 Ko de memoire.

Ils inventerent le **CBOR** — Concise Binary Object Representation. Le meme modele de donnees que JSON, mais encode en binaire. Compact, rapide, extensible. Publie en tant que RFC 7049 par l'IETF en octobre 2013.

Le CBOR n'etait pas concu pour le web. Il etait concu pour les machines contraintes. Des thermometres, des serrures de porte, des compteurs electriques. Des choses qui comptent chaque octet.

### Chapitre 2 — Maturite et standardisation (2020)

Sept ans plus tard, le CBOR avait fait ses preuves. Adopte par CoAP (le HTTP de l'IoT), integre dans WebAuthn (les cles d'authentification FIDO2), utilise par COSE pour les signatures cryptographiques. Des millions d'appareils parlaient CBOR sans que personne ne le remarque.

En decembre 2020, la RFC 8949 remplacait la specification originale. Clarifications, corrections, encodage deterministe. Le CBOR devenait un Standard Internet — le plus haut niveau de validation a l'IETF.

Mais le web continuait a l'ignorer. Les sites envoyaient toujours des megaoctets de HTML, CSS et JavaScript pour afficher quelques paragraphes de texte.

### Chapitre 3 — Le tournant : les agents IA arrivent (2024-2026)

Puis tout a change. Les agents IA se sont mis a parcourir le web. Pas avec des yeux et une souris — avec des requetes HTTP et des parseurs. Et ce qu'ils ont trouve etait absurde :

- **2,86 Mo** en moyenne par page en 2025 (HTTP Archive)
- **86 requetes** pour afficher une seule page
- **95% de bruit** (CSS, JavaScript, tracking, pubs) pour **5% de contenu utile**

Un agent IA qui veut lire le menu d'un restaurant doit telecharger 3 Mo de code React, executer du JavaScript, attendre 47 appels API, combattre les bannieres de cookies, et analyser un DOM de 2 000 noeuds. Pour trouver : adresse, telephone, horaires, carte. 200 octets d'information utile enterres sous des megaoctets de decoration.

### Chapitre 4 — CBOR-Web : un fichier, tout le site (2026)

L'idee etait evidente, pourtant personne ne l'avait fait : **et si un site web entier tenait dans un seul fichier binaire a la racine ?**

`index.cbor` — place a cote de `index.html`. Meme adresse, format different. Le HTML pour les humains, le CBOR pour les machines.

Un fichier. Une requete. Zero JavaScript. Zero CSS. Juste du contenu structure : titres, paragraphes, listes, tableaux, liens, boutons d'action. Tout ce dont un agent IA a besoin, rien de ce dont il n'a pas besoin.

Une page Shopify de 160 Ko devient 8 Ko de CBOR. Un site complet de 84 pages tient dans 700 Ko. Quinze fois plus leger. Une requete au lieu de quatre-vingt-six.

### Chapitre 5 — L'imperatif ecologique

Les chiffres sont vertigineux :

- Les centres de donnees ont consomme **460 TWh** d'electricite en 2022 — prevision de **620 a 1 050 TWh** d'ici 2026
- La consommation energetique du numerique augmente de **9% par an**
- Un site web moyen avec 10 000 visites mensuelles emet **31,5 kg de CO2** par an
- Le poids des pages web a augmente de **+203%** depuis 2015

Chaque octet inutile transfere est de l'energie gaspillee. Chaque animation CSS qu'aucune machine ne verra jamais est un serveur qui tourne, un cable qui transporte des donnees, un routeur qui achemine des paquets — pour rien.

CBOR-Web ne rend pas seulement le web plus rapide pour l'IA. Il le rend **plus leger pour la planete**.

| | HTML traditionnel | CBOR-Web |
|---|---|---|
| Poids moyen par page | 2,86 Mo | 5-50 Ko |
| Requetes par page | 50-100 | 1 |
| Ratio contenu utile | ~5% | 100% |
| Energie par lecture | Elevee | Minimale |

Si chaque site proposait un `index.cbor` a cote de son `index.html`, le volume de donnees transferees par les agents IA chuterait d'un facteur **10 a 50**. Pas par la compression. Par l'elimination de l'inutile.

### Chapitre 6 — Un standard qui appartient a tous

Le protocole de lecture CBOR-Web est publie sous **CC0 — Domaine Public**. Pas de licence a payer, pas de permission a demander, pas de brevet a craindre. Tout le monde peut lire un `index.cbor`. Le protocole de lecture appartient a l'humanite.

Ce qui cree de la valeur, ce n'est pas la lecture — c'est la **creation**. Transformer un site HTML en un `index.cbor` propre, structure, optimise demande de l'intelligence, des outils, du savoir-faire. C'est la que les editeurs interviennent. C'est la que l'ecosysteme se construit.

Comme le PDF : tout le monde peut lire. Les outils qui creent sont la ou vit la valeur.

---

<a id="espanol"></a>
## 🇪🇸 Espanol

### Capitulo 1 — Dos investigadores, una idea (2013)

En 2013, dos investigadores — Carsten Bormann de la Universidad de Bremen y Paul Hoffman de ICANN — enfrentaban un problema concreto: los objetos conectados (sensores IoT, tarjetas inteligentes, controladores industriales) necesitaban intercambiar datos estructurados, pero JSON era demasiado pesado. Demasiado verboso. Demasiado lento para analizar en un procesador con 8 KB de memoria.

Inventaron **CBOR** — Concise Binary Object Representation. El mismo modelo de datos que JSON, pero codificado en binario. Compacto, rapido, extensible. Publicado como RFC 7049 por la IETF en octubre de 2013.

CBOR no fue disenado para la web. Fue disenado para maquinas limitadas. Termometros, cerraduras, contadores electricos. Cosas que cuentan cada byte.

### Capitulo 2 — Madurez y estandarizacion (2020)

Siete anos despues, CBOR habia demostrado su valor. Adoptado por CoAP, integrado en WebAuthn, utilizado por COSE para firmas criptograficas. Millones de dispositivos hablaban CBOR sin que nadie se diera cuenta.

En diciembre de 2020, RFC 8949 reemplazo la especificacion original. CBOR se convirtio en un Estandar de Internet — el nivel mas alto de validacion en la IETF.

Pero la web seguia ignorandolo.

### Capitulo 3 — El punto de inflexion: llegan los agentes IA (2024-2026)

Los agentes IA comenzaron a navegar la web. Y lo que encontraron fue absurdo: **2,86 MB** por pagina, **86 peticiones** para mostrar una sola pagina, **95% ruido** para **5% de contenido util**.

### Capitulo 4 — CBOR-Web: un archivo, todo el sitio (2026)

`index.cbor` — junto a `index.html`. Un archivo. Una peticion. Cero JavaScript. Solo contenido estructurado.

Una pagina Shopify de 160 KB se convierte en 8 KB de CBOR. Un sitio completo de 84 paginas cabe en 700 KB.

### Capitulo 5 — El imperativo ecologico

- Los centros de datos consumieron **460 TWh** en 2022 — prevision de **620-1.050 TWh** para 2026
- El consumo energetico digital crece **9% anual**
- El peso de las paginas web ha aumentado **+203%** desde 2015

Cada byte innecesario es energia desperdiciada. CBOR-Web hace la web **mas ligera para el planeta**.

### Capitulo 6 — Un estandar que pertenece a todos

Protocolo de lectura publicado bajo **CC0 — Dominio Publico**. Leer es gratis. Crear es donde vive el valor.

---

<a id="中文"></a>
## 🇨🇳 中文

### 第一章 — 两位研究者，一个想法（2013）

2013年，两位研究者 — 不来梅大学的 Carsten Bormann 和 ICANN 的 Paul Hoffman — 面临一个具体问题：物联网设备需要交换结构化数据，但 JSON 太重了。他们发明了 **CBOR**（简洁二进制对象表示法）。与 JSON 相同的数据模型，但以二进制编码。于 2013 年 10 月作为 RFC 7049 发布。

### 第四章 — CBOR-Web：一个文件，整个网站（2026）

`index.cbor` — 放在 `index.html` 旁边。一个文件，一次请求，零 JavaScript。2025 年平均网页 **2.86 MB**，CBOR-Web 仅需 **5-50 KB**。

### 第五章 — 生态责任

数据中心 2022 年消耗 **460 TWh** 电力，预计 2026 年达到 **620-1,050 TWh**。每个无用字节都是能源浪费。CBOR-Web 让网络对地球**更轻**。

协议以 **CC0 公共领域** 发布。阅读自由，无限制。

---

<a id="العربية"></a>
## 🇸🇦 العربية

### الفصل الأول — باحثان، فكرة واحدة (2013)

في عام 2013، واجه باحثان — كارستن بورمان من جامعة بريمن وبول هوفمان من ICANN — مشكلة ملموسة: أجهزة إنترنت الأشياء كانت بحاجة لتبادل بيانات مهيكلة، لكن JSON كان ثقيلاً جداً. اخترعا **CBOR** — نموذج بيانات JSON نفسه، لكن بترميز ثنائي. نُشر كـ RFC 7049 في أكتوبر 2013.

### الفصل الرابع — CBOR-Web: ملف واحد، الموقع بالكامل (2026)

`index.cbor` — بجانب `index.html`. ملف واحد. طلب واحد. بدون JavaScript. متوسط صفحة الويب **2.86 ميجابايت** في 2025، CBOR-Web يحتاج فقط **5-50 كيلوبايت**.

### الفصل الخامس — الضرورة البيئية

استهلكت مراكز البيانات **460 تيراواط ساعة** في 2022. كل بايت غير ضروري هو طاقة مهدرة.

البروتوكول منشور تحت **CC0 — ملكية عامة**. القراءة حرة للجميع.

---

<a id="हिन्दी"></a>
## 🇮🇳 हिन्दी

### अध्याय 1 — दो शोधकर्ता, एक विचार (2013)

2013 में, दो शोधकर्ताओं — ब्रेमेन विश्वविद्यालय के Carsten Bormann और ICANN के Paul Hoffman — ने एक समस्या का सामना किया: IoT उपकरणों को संरचित डेटा का आदान-प्रदान करना था, लेकिन JSON बहुत भारी था। उन्होंने **CBOR** का आविष्कार किया। RFC 7049 के रूप में अक्टूबर 2013 में प्रकाशित।

### अध्याय 4 — CBOR-Web: एक फ़ाइल, पूरी वेबसाइट (2026)

`index.cbor` — `index.html` के बगल में। एक फ़ाइल। एक अनुरोध। शून्य JavaScript। 2025 में औसत वेबपेज **2.86 MB**, CBOR-Web केवल **5-50 KB**।

### अध्याय 5 — पारिस्थितिक अनिवार्यता

डेटा केंद्रों ने 2022 में **460 TWh** बिजली की खपत की। हर अनावश्यक बाइट बर्बाद ऊर्जा है।

प्रोटोकॉल **CC0 — सार्वजनिक डोमेन** के तहत प्रकाशित। पढ़ना मुफ्त, बिना प्रतिबंध।

---

<a id="portugues"></a>
## 🇧🇷 Portugues

### Capitulo 1 — Dois pesquisadores, uma ideia (2013)

Em 2013, Carsten Bormann (Universidade de Bremen) e Paul Hoffman (ICANN) inventaram o **CBOR** — o mesmo modelo de dados do JSON, mas codificado em binario. Publicado como RFC 7049 em outubro de 2013. Projetado para IoT, nao para a web.

### Capitulo 4 — CBOR-Web: um arquivo, o site inteiro (2026)

`index.cbor` — ao lado de `index.html`. Um arquivo. Uma requisicao. Zero JavaScript. Pagina media em 2025: **2,86 MB**. CBOR-Web: **5-50 KB**.

### Capitulo 5 — O imperativo ecologico

Data centers consumiram **460 TWh** em 2022. Cada byte desnecessario e energia desperdicada. CBOR-Web torna a web **mais leve para o planeta**.

Protocolo publicado sob **CC0 — Dominio Publico**.

---

<a id="русский"></a>
## 🇷🇺 Русский

### Глава 1 — Два исследователя, одна идея (2013)

В 2013 году Карстен Борман (Бременский университет) и Пол Хоффман (ICANN) создали **CBOR** — ту же модель данных, что и JSON, но в бинарной кодировке. Опубликован как RFC 7049 в октябре 2013 года. Разработан для IoT.

### Глава 4 — CBOR-Web: один файл — весь сайт (2026)

`index.cbor` — рядом с `index.html`. Один файл. Один запрос. Без JavaScript. Средняя веб-страница в 2025: **2,86 МБ**. CBOR-Web: **5-50 КБ**.

### Глава 5 — Экологический императив

Дата-центры потребили **460 ТВт·ч** в 2022 году. Каждый лишний байт — потраченная энергия.

Протокол опубликован под **CC0 — общественное достояние**.

---

<a id="日本語"></a>
## 🇯🇵 日本語

### 第1章 — 二人の研究者、一つのアイデア（2013年）

2013年、Carsten Bormann（ブレーメン大学）と Paul Hoffman（ICANN）は **CBOR** を発明しました。JSONと同じデータモデルをバイナリで符号化。RFC 7049として2013年10月に公開。IoT向けに設計されました。

### 第4章 — CBOR-Web：1ファイルでサイト全体（2026年）

`index.cbor` — `index.html` の隣に配置。1ファイル、1リクエスト、JavaScript不要。2025年の平均ウェブページ **2.86 MB**、CBOR-Webなら **5〜50 KB**。

### 第5章 — 環境への責務

データセンターは2022年に **460 TWh** を消費。不要なバイトはすべてエネルギーの無駄です。

プロトコルは **CC0（パブリックドメイン）** で公開。自由に読めます。

---

<a id="deutsch"></a>
## 🇩🇪 Deutsch

### Kapitel 1 — Zwei Forscher, eine Idee (2013)

2013 erfanden Carsten Bormann (Universitat Bremen) und Paul Hoffman (ICANN) **CBOR** — dasselbe Datenmodell wie JSON, aber binar kodiert. Veroffentlicht als RFC 7049 im Oktober 2013. Entwickelt fur IoT.

### Kapitel 4 — CBOR-Web: eine Datei, die ganze Website (2026)

`index.cbor` — neben `index.html`. Eine Datei. Eine Anfrage. Kein JavaScript. Durchschnittliche Webseite 2025: **2,86 MB**. CBOR-Web: **5-50 KB**.

### Kapitel 5 — Der okologische Imperativ

Rechenzentren verbrauchten 2022 **460 TWh**. Jedes unnotige Byte ist verschwendete Energie. CBOR-Web macht das Web **leichter fur den Planeten**.

Protokoll unter **CC0 — Gemeinfreiheit** veroffentlicht.

---

<a id="한국어"></a>
## 🇰🇷 한국어

### 1장 — 두 연구자, 하나의 아이디어 (2013)

2013년, Carsten Bormann(브레멘 대학)과 Paul Hoffman(ICANN)이 **CBOR**를 발명했습니다. JSON과 동일한 데이터 모델을 바이너리로 인코딩. 2013년 10월 RFC 7049로 발표.

### 4장 — CBOR-Web: 하나의 파일, 전체 웹사이트 (2026)

`index.cbor` — `index.html` 옆에 배치. 파일 하나, 요청 하나, JavaScript 없음. 2025년 평균 웹페이지 **2.86 MB**, CBOR-Web은 **5-50 KB**.

### 5장 — 생태적 의무

데이터센터는 2022년 **460 TWh**를 소비했습니다. 불필요한 바이트는 모두 에너지 낭비입니다.

프로토콜은 **CC0 — 퍼블릭 도메인**으로 공개되었습니다.

---

<a id="bahasa-indonesia"></a>
## 🇮🇩 Bahasa Indonesia

### Bab 1 — Dua peneliti, satu ide (2013)

Pada 2013, Carsten Bormann (Universitas Bremen) dan Paul Hoffman (ICANN) menciptakan **CBOR** — model data yang sama dengan JSON, tapi dikodekan dalam biner. Diterbitkan sebagai RFC 7049 pada Oktober 2013. Dirancang untuk IoT.

### Bab 4 — CBOR-Web: satu file, seluruh situs (2026)

`index.cbor` — di samping `index.html`. Satu file. Satu permintaan. Tanpa JavaScript. Rata-rata halaman web 2025: **2,86 MB**. CBOR-Web: **5-50 KB**.

### Bab 5 — Keharusan ekologis

Pusat data mengonsumsi **460 TWh** pada 2022. Setiap byte yang tidak perlu adalah energi yang terbuang.

Protokol diterbitkan di bawah **CC0 — Domain Publik**.

---

## Sources / References

- [RFC 7049 — CBOR (2013)](https://www.rfc-editor.org/rfc/rfc7049.html) — Carsten Bormann, Paul Hoffman
- [RFC 8949 — CBOR (2020)](https://www.rfc-editor.org/rfc/rfc8949.html) — Internet Standard
- [CBOR — Wikipedia](https://en.wikipedia.org/wiki/CBOR)
- [HTTP Archive — Page Weight 2025](https://almanac.httparchive.org/en/2025/page-weight) — Median 2.86 MB
- [IEA — Data centres energy consumption](https://www.iea.org/reports/global-energy-review-2025/co2-emissions) — 460 TWh (2022)
- [Sustainable Web Design](https://sustainablewebdesign.org/estimating-digital-emissions/) — Carbon footprint methodology
- [Website Carbon Calculator](https://www.websitecarbon.com/) — Per-site CO2 estimation
