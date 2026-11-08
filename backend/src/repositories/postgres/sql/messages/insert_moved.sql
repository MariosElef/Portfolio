-- A message sent before the inbox moved here, keeping when it was sent.
INSERT INTO messages (id, name, email, body, user_agent, created_at)
VALUES ($1, $2, $3, $4, $5, $6)
