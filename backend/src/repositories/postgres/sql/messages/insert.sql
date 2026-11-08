-- A message sent through the contact form.
INSERT INTO messages (id, name, email, body, user_agent)
VALUES ($1, $2, $3, $4, $5)
