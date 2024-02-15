SELECT *, search_score / max_score[0].max_score + (1 + sim) / 2 AS total_score 
  FROM (
    SELECT 
      *,
      (SELECT math::max(search::score(1)) AS max_score 
        FROM notes 
          WHERE owner_telegram_id == $id AND (text @1@ $search_text OR 1)
          GROUP ALL
      ) AS max_score,
      vector::similarity::cosine(embedding, $search_embedding) AS sim,
      search::score(1) AS search_score 
      FROM notes
        WHERE owner_telegram_id == $id AND (text @1@ $search_text OR 1)
  )
ORDER BY total_score DESC;
