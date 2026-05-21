---
name: Feature request
about: Propose an addition to the spec or to the tooling
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

**Problem**
What problem does this feature solve? Who hits it?

**Proposed solution**
A clear description of what you want to add. If it changes the wire format,
show the proposed CBOR structure using the 7-keys layout:

```cbor-diag
55799({
  0: "cbor-web",
  1: 3,
  2: { ... },
  3: { ... },
  4: { ... },
  5: [
    {
      "path": "/...",
      "title": "...",
      "lang": "en",
      "access": "T2",
      "content": [
        { "t": "h", "l": 1, "v": "..." }
      ]
    }
  ],
  6: { ... }
})
```

**Alternatives considered**
Other approaches you thought about and why you rejected them.

**Use case**
Who benefits — crawlers, IoT clients, publishers, …

**Backwards compatibility**
- [ ] Backwards-compatible (new optional field, new block type, etc.)
- [ ] Breaking (existing parsers will reject the file) — explain why this is unavoidable

**References**
Similar features in other formats, prior art, related discussions.
