-- The tools listed in the stack: those with a category, in its order.
SELECT
    tool_categories.name AS category,
    tool_categories.note AS category_note,
    tools.name,
    tools.icon
FROM tools
INNER JOIN tool_categories ON tools.category_id = tool_categories.id
ORDER BY tool_categories.sort, tool_categories.name, tools.sort, tools.name
