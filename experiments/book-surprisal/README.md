# book-surprisal

Per-token surprisal over a book's OCR'd pages, first pass against refined,
scored through the safetensors model with each page alone, and the
pipeline's own flagged artifacts tested against their page's distribution.
The question is whether surprisal spikes land on OCR mistakes, and the
answer of 2026-09-03 on I, Robot, under the script's occurrence policy, is
that flagged artifacts reach their page's top five percent 39.7 percent of
the time against 8.9 for random occurrences and 29.9 for rare real words,
words occurring once in the whole book. Four and a half times the random
rate and 1.3 times the rare-word rate: the signal finds rarity and not
wrongness. An earlier analysis outside the script, scoring first
occurrences, read 12.7 and 23.6 for the two controls, and the review of
PR #417 found that policy biased, so the script's figures are the ones. The agent-side route to the same measurement is
the re-feed drive under the surprisal election, owed as its own act.
