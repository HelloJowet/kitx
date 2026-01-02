CREATE TABLE article (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    tenant_id INT NOT NULL DEFAULT 0,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    views INT DEFAULT 0,
    deleted BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE article_tag (
    article_id INT NOT NULL,
    share_seq INT NOT NULL,
    tenant_id INT NOT NULL DEFAULT 0,
    tag VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (article_id, share_seq),
    FOREIGN KEY (article_id) REFERENCES article(id)
);
