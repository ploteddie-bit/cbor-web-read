# CBOR-Web — The Story

The history behind the protocol, in three languages: [English](#english) ·
[Français](#francais) · [Español](#espanol).

For the technical specification, see
[CBOR-WEB-SPEC-v3.0.md](CBOR-WEB-SPEC-v3.0.md).

---

<a id="english"></a>
## English

### Chapter 1 — Two researchers, one idea (2013)

In 2013, Carsten Bormann (University of Bremen) and Paul Hoffman (ICANN)
faced a concrete problem: connected objects — IoT sensors, smart cards,
industrial controllers — needed to exchange structured data, but JSON was
too heavy. Too verbose. Too slow to parse on a processor with 8 KB of
memory.

They invented **CBOR** (Concise Binary Object Representation): the same data
model as JSON, but encoded in binary. Compact, fast, extensible. Published
as RFC 7049 by the IETF in October 2013.

CBOR was not designed for the web. It was designed for constrained machines.
Thermometers, door locks, electricity meters. Things that count every byte.

### Chapter 2 — Maturity and standardisation (2020)

Seven years later, CBOR had proven itself. Adopted by CoAP (the HTTP of
IoT), integrated into WebAuthn (FIDO2 authentication keys), used by COSE for
cryptographic signatures. Millions of devices spoke CBOR without anyone
noticing.

In December 2020, [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html)
replaced the original specification. Clarifications, corrections,
deterministic encoding. CBOR became an Internet Standard — the highest level
of validation at the IETF.

But the web continued to ignore it. Sites kept sending megabytes of HTML,
CSS, and JavaScript to display a few paragraphs of text.

### Chapter 3 — AI agents arrive (2024–2026)

Then things shifted. Autonomous agents started browsing the web. Not with
eyes and a mouse — with HTTP requests and parsers. What they found:

- ~2.86 MB average per page in 2025 ([HTTP Archive](https://almanac.httparchive.org/en/2025/page-weight))
- ~86 requests to display a single page
- A small fraction of that payload is useful content; the rest is styling,
  scripts, analytics, and ads

An agent that wants to read a restaurant's address, phone number and opening
hours must download megabytes of unrelated code, execute JavaScript, fight
through cookie banners and parse a DOM with thousands of nodes. The signal-
to-noise ratio is poor.

### Chapter 4 — CBOR-Web (2026)

The idea: place a binary, machine-readable mirror of a site at the same
address as `index.html`. Call it `index.cbor`. Humans read HTML, machines
read CBOR.

One file. One request. No JavaScript, no CSS, no DOM. Just structured
content: headings, paragraphs, lists, tables, links, calls to action.

A typical content-heavy page that ships as 1–3 MB of HTML can usually be
expressed as 5–50 KB of CBOR-Web. The size gap is not magic — it is the
difference between transmitting structured content (CBOR-Web) and
transmitting a layout instruction set plus assets (HTML).

### Chapter 5 — The ecological angle

| | Traditional HTML | CBOR-Web |
|---|---|---|
| Page weight | 1–3 MB typical | 5–50 KB typical |
| Requests | 50–100 | 1 |
| Useful-content ratio | Low | High |
| Energy per read | Higher | Lower |

Data centres consumed ~415 TWh of electricity in 2024
([IEA, "Energy and AI", 2025](https://www.iea.org/reports/energy-and-ai)), with projections in the
high hundreds of TWh by 2030. Less data transferred and less compute on
both ends means less energy. The gain per request is small; multiplied by
billions of agent requests per day, it becomes material.

### Chapter 6 — A standard that belongs to everyone

CBOR-Web is published under **CC0 1.0** — public domain. No licence to pay,
no permission to ask, no patent to fear. Reading an `index.cbor` is free.
Anyone may fork, extend, or build commercial services on top of the spec.

Like PDF: anyone can read. Anyone can write. The value lives in the tooling
ecosystem, not in the gatekeeping of the format.

---

<a id="francais"></a>
## Français

### Chapitre 1 — Deux chercheurs, une idée (2013)

En 2013, Carsten Bormann (Université de Brême) et Paul Hoffman (ICANN)
faisaient face à un problème concret : les objets connectés — capteurs IoT,
cartes à puce, contrôleurs industriels — devaient échanger des données
structurées, mais le JSON était trop lourd. Trop verbeux. Trop lent à
analyser sur un processeur de 8 Ko de mémoire.

Ils inventent **CBOR** (Concise Binary Object Representation) : le même
modèle de données que JSON, encodé en binaire. Compact, rapide, extensible.
Publié en tant que RFC 7049 par l'IETF en octobre 2013.

CBOR n'était pas conçu pour le web. Il était conçu pour les machines
contraintes. Thermomètres, serrures, compteurs électriques. Des choses qui
comptent chaque octet.

### Chapitre 2 — Maturité et standardisation (2020)

Sept ans plus tard, CBOR a fait ses preuves. Adopté par CoAP (le HTTP de
l'IoT), intégré à WebAuthn (clés FIDO2), utilisé par COSE pour les
signatures cryptographiques. Des millions d'appareils parlent CBOR sans que
personne ne le remarque.

En décembre 2020, la RFC 8949 remplace la spécification originale.
Clarifications, corrections, encodage déterministe. CBOR devient un Standard
Internet — le plus haut niveau de validation à l'IETF.

Mais le web continue à l'ignorer. Les sites envoient toujours des
mégaoctets de HTML, CSS et JavaScript pour afficher quelques paragraphes de
texte.

### Chapitre 3 — Les agents IA arrivent (2024–2026)

Puis tout change. Les agents autonomes se mettent à parcourir le web. Pas
avec des yeux et une souris — avec des requêtes HTTP et des parseurs. Ce
qu'ils trouvent :

- ~2,86 Mo en moyenne par page en 2025 (HTTP Archive)
- ~86 requêtes pour afficher une seule page
- Une petite fraction de cette charge utile est du contenu utile ; le reste,
  c'est du style, des scripts, de l'analytics et de la publicité

Un agent qui veut lire l'adresse, le téléphone et les horaires d'un
restaurant doit télécharger des mégaoctets de code sans rapport, exécuter du
JavaScript, traverser des bannières de cookies et analyser un DOM de
plusieurs milliers de nœuds. Le rapport signal sur bruit est faible.

### Chapitre 4 — CBOR-Web (2026)

L'idée : placer un miroir binaire, lisible par les machines, à la même
adresse qu'`index.html`. L'appeler `index.cbor`. Les humains lisent l'HTML,
les machines lisent le CBOR.

Un fichier. Une requête. Pas de JavaScript, pas de CSS, pas de DOM. Juste
du contenu structuré : titres, paragraphes, listes, tableaux, liens, appels
à l'action.

Une page typique qui pèse 1 à 3 Mo d'HTML peut généralement s'exprimer en 5
à 50 Ko de CBOR-Web. L'écart de taille n'est pas magique — c'est la
différence entre transmettre du contenu structuré (CBOR-Web) et transmettre
un jeu d'instructions de mise en page plus des actifs (HTML).

### Chapitre 5 — L'angle écologique

| | HTML traditionnel | CBOR-Web |
|---|---|---|
| Poids par page | 1–3 Mo typique | 5–50 Ko typique |
| Requêtes | 50–100 | 1 |
| Ratio contenu utile | Faible | Élevé |
| Énergie par lecture | Plus élevée | Plus faible |

Les centres de données ont consommé ~415 TWh d'électricité en 2024 (AIE,
« Energy and AI », 2025), avec des projections dans la centaine de TWh
d'ici 2030. Moins de données transférées et moins de calcul aux deux
extrémités, c'est moins d'énergie. Le gain par requête est faible ;
multiplié par les milliards de requêtes d'agents quotidiennes, il devient
significatif.

### Chapitre 6 — Un standard qui appartient à tous

CBOR-Web est publié sous **CC0 1.0** — domaine public. Pas de licence à
payer, pas de permission à demander, pas de brevet à craindre. Lire un
`index.cbor` est libre. N'importe qui peut forker, étendre, ou construire
des services commerciaux par-dessus la spec.

Comme le PDF : tout le monde peut lire. Tout le monde peut écrire. La
valeur vit dans l'outillage, pas dans le contrôle du format.

---

<a id="espanol"></a>
## Español

### Capítulo 1 — Dos investigadores, una idea (2013)

En 2013, Carsten Bormann (Universidad de Bremen) y Paul Hoffman (ICANN)
enfrentaban un problema concreto: los objetos conectados — sensores IoT,
tarjetas inteligentes, controladores industriales — necesitaban
intercambiar datos estructurados, pero JSON era demasiado pesado. Demasiado
verboso. Demasiado lento de analizar en un procesador con 8 KB de memoria.

Inventaron **CBOR** (Concise Binary Object Representation): el mismo modelo
de datos que JSON, codificado en binario. Compacto, rápido, extensible.
Publicado como RFC 7049 por la IETF en octubre de 2013.

CBOR no fue diseñado para la web. Fue diseñado para máquinas limitadas.
Termómetros, cerraduras, contadores eléctricos. Cosas que cuentan cada
byte.

### Capítulo 2 — Madurez y estandarización (2020)

Siete años después, CBOR había demostrado su valor. Adoptado por CoAP,
integrado en WebAuthn, utilizado por COSE para firmas criptográficas.
Millones de dispositivos hablaban CBOR sin que nadie se diera cuenta.

En diciembre de 2020, RFC 8949 reemplazó la especificación original. CBOR
se convirtió en un Estándar de Internet — el nivel más alto de validación
en la IETF.

Pero la web seguía ignorándolo.

### Capítulo 3 — Los agentes IA (2024–2026)

Los agentes autónomos comenzaron a navegar la web. No con ojos y ratón —
con peticiones HTTP y parsers. Lo que encontraron:

- ~2,86 MB por página en 2025 (HTTP Archive)
- ~86 peticiones para mostrar una sola página
- Una pequeña fracción de esa carga es contenido útil; el resto es estilo,
  scripts, analítica y publicidad

### Capítulo 4 — CBOR-Web (2026)

`index.cbor` junto a `index.html`. Un archivo. Una petición. Sin
JavaScript, sin CSS. Solo contenido estructurado: títulos, párrafos,
listas, tablas, enlaces. Una página típica de 1–3 MB de HTML suele caber en
5–50 KB de CBOR-Web.

### Capítulo 5 — La cuestión ecológica

Los centros de datos consumieron ~415 TWh en 2024 (IEA). Menos datos
transferidos y menos cómputo en ambos extremos significa menos energía. El
ahorro por petición es pequeño; multiplicado por miles de millones de
peticiones diarias de agentes, se vuelve significativo.

### Capítulo 6 — Un estándar de todos

CBOR-Web se publica bajo **CC0 1.0** — dominio público. Sin licencia,
sin permiso, sin patentes. Cualquiera puede leer, escribir, forkar y
construir servicios comerciales sobre la especificación.

---

## Sources

- [RFC 7049](https://www.rfc-editor.org/rfc/rfc7049.html) — CBOR (2013),
  Carsten Bormann, Paul Hoffman
- [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) — CBOR (2020),
  Internet Standard
- [CBOR — Wikipedia](https://en.wikipedia.org/wiki/CBOR)
- [HTTP Archive — Page Weight 2025](https://almanac.httparchive.org/en/2025/page-weight)
- [IEA — Energy and AI](https://www.iea.org/reports/energy-and-ai) —
  electricity consumption of data centres
- [Sustainable Web Design](https://sustainablewebdesign.org/estimating-digital-emissions/) — carbon
  footprint methodology

## Author

Eddie Plot, Burriana, 2026. Early drafts were developed in dialogue with
the Claude assistant from Anthropic, which contributed to the formalisation
of the structure and encoding rules.
