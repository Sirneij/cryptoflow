pub const QUESTION_AUTHOR_WITH_TAGS_QUERY: &str = "
SELECT
    q.id,
    q.title,
    q.slug,
    q.content,
    q.raw_content,
    q.created_at,
    q.updated_at,
    JSON_AGG(
        JSON_BUILD_OBJECT(
            'id', t.id,
            'name', t.name,
            'symbol', t.symbol
        )
    ) as tags_json,
    u.id as user_id,
    u.email as user_email,
    u.first_name as user_first_name,
    u.last_name as user_last_name,
    u.is_active as user_is_active,
    u.is_staff as user_is_staff,
    u.is_superuser as user_is_superuser,
    u.thumbnail as user_thumbnail,
    u.date_joined as user_date_joined
FROM questions q
LEFT JOIN users u ON q.author = u.id
LEFT JOIN question_tags qt ON q.id = qt.question
LEFT JOIN tags t ON qt.tag = t.id
WHERE q.id = $1
GROUP BY q.id, u.id;
";

pub const QUESTION_AUTHOR_WITH_TAGS_QUERY_ALL: &str = "
SELECT
    q.id,
    q.title,
    q.slug,
    q.content,
    q.raw_content,
    q.created_at,
    q.updated_at,
    JSON_AGG(
        JSON_BUILD_OBJECT(
            'id', t.id,
            'name', t.name,
            'symbol', t.symbol
        )
    ) as tags_json,
    u.id as user_id,
    u.email as user_email,
    u.first_name as user_first_name,
    u.last_name as user_last_name,
    u.is_active as user_is_active,
    u.is_staff as user_is_staff,
    u.is_superuser as user_is_superuser,
    u.thumbnail as user_thumbnail,
    u.date_joined as user_date_joined
FROM questions q
LEFT JOIN users u ON q.author = u.id
LEFT JOIN question_tags qt ON q.id = qt.question
LEFT JOIN tags t ON qt.tag = t.id
GROUP BY q.id, u.id;
";

pub const ANSWER_AUTHOR_QUERY: &str = "
SELECT
    a.id,
    a.content,
    a.raw_content,
    a.created_at,
    a.updated_at,
    u.id as user_id,
    u.email as user_email,
    u.first_name as user_first_name,
    u.last_name as user_last_name,
    u.is_active as user_is_active,
    u.is_staff as user_is_staff,
    u.is_superuser as user_is_superuser,
    u.thumbnail as user_thumbnail,
    u.date_joined as user_date_joined
FROM answers a
LEFT JOIN users u ON a.author = u.id
WHERE a.id = $1;
";

pub const ANSWER_AUTHOR_QUERY_VIA_QUESTION_ID: &str = "
SELECT
    a.id,
    a.content,
    a.raw_content,
    a.created_at,
    a.updated_at,
    u.id as user_id,
    u.email as user_email,
    u.first_name as user_first_name,
    u.last_name as user_last_name,
    u.is_active as user_is_active,
    u.is_staff as user_is_staff,
    u.is_superuser as user_is_superuser,
    u.thumbnail as user_thumbnail,
    u.date_joined as user_date_joined
FROM answers a
LEFT JOIN users u ON a.author = u.id
WHERE a.question = $1;
";
