USE NS eau DB note;

DEFINE TABLE notes;

DEFINE FIELD text ON TABLE notes TYPE string;
DEFINE FIELD owner_telegram_id ON TABLE notes TYPE int;
DEFINE FIELD embedding ON TABLE notes TYPE array<float>;

DEFINE ANALYZER ascii TOKENIZERS class FILTERS ascii;
DEFINE INDEX noteText ON TABLE notes COLUMNS text SEARCH ANALYZER ascii BM25 HIGHLIGHTS;

DEFINE EVENT add_note ON TABLE notes WHEN $event = "CREATE" THEN {
  LET $connectable = (SELECT * FROM notes WHERE vector::similarity::cosine(embedding, $after.embedding) > 0.7 AND id != $after.id);
  RELATE $connectable->connected->$after;
  RELATE $after->connected->$connectable;
};
