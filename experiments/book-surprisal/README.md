# book-surprisal

Per-token surprisal over a book's OCR'd pages, first pass against refined,
scored through the safetensors model with each page alone, and the
pipeline's own flagged artifacts tested against their page's distribution.
The question is whether surprisal spikes land on OCR mistakes, and the
answer of 2026-09-03 on I, Robot is that they do three times as often as on
random words and 1.7 times as often as on rare real words: the signal finds
rarity and not wrongness. The agent-side route to the same measurement is
the re-feed drive under the surprisal election, owed as its own act.
