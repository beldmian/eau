DEFINE ANALYZER ascii TOKENIZERS class FILTERS ascii;
DEFINE INDEX noteText ON TABLE notes COLUMNS text SEARCH ANALYZER ascii BM25 HIGHLIGHTS;
