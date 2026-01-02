CREATE TABLE article (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tenant_id INTEGER NOT NULL DEFAULT 0,
    title TEXT NOT NULL,
    content TEXT,
    views INTEGER DEFAULT 0,
    deleted INTEGER DEFAULT 0 CHECK(deleted IN (0, 1)),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'draft'
);
CREATE TABLE article_tag (
    article_id INTEGER NOT NULL,
    share_seq INTEGER NOT NULL,
    tenant_id INTEGER NOT NULL DEFAULT 0,
    tag TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (article_id, share_seq),
    FOREIGN KEY (article_id) REFERENCES article(id)
);
