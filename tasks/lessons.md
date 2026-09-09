# Lessons

- When extracting stateful logic into a pure function, characterise sequences and state-update policy as well as independent outputs; a detected display bucket is not necessarily the last real project number.
- Persistence failures on shutdown must be logged without blocking the quit path; test the failure path, not only successful round trips.
- Keep release artifact names separate from stable installed filenames; trace upgrades through disk paths, running processes, shortcuts, and Run keys before unifying identity constants.
- When deleting a whole method with a regex, anchor the end to the next `def` (lookahead), never to the first blank line; a blank line inside the body leaves an orphaned half-method that still parses.
- Store segment timestamps as epoch floats, not ISO strings: round-tripping ISO through `fromisoformat().timestamp()` raises OSError on Windows for pre-1970 test clocks and costs a parse per comparison.
