CREATE TABLE article (
    id SERIAL NOT NULL PRIMARY KEY,
    tenant_id INTEGER NOT NULL DEFAULT 0,
    title TEXT NOT NULL,
    content TEXT,
    views INTEGER DEFAULT 0,
    deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(255) NOT NULL DEFAULT 'draft'
);

CREATE TABLE article_tag (
    article_id INTEGER NOT NULL,
    share_seq INTEGER NOT NULL,
    tenant_id INTEGER NOT NULL DEFAULT 0,
    tag TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (article_id, share_seq),
    FOREIGN KEY (article_id) REFERENCES article(id)
);
