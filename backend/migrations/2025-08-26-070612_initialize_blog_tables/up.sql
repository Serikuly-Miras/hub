-- Create the pg_trgm extension
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Post statuses table
CREATE TABLE
    post_statuses (
        id SERIAL PRIMARY KEY,
        name VARCHAR(63) UNIQUE NOT NULL
    );

CREATE INDEX idx_post_statuses_name ON post_statuses (name);

-- Posts table
CREATE TABLE
    posts (
        id SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        status INTEGER REFERENCES post_statuses (id),
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT NOW (),
        updated_at TIMESTAMP DEFAULT NOW (),
        deleted_at TIMESTAMP
    );

CREATE INDEX idx_posts_title_trgm ON posts USING GIN (title gin_trgm_ops);

CREATE INDEX idx_posts_content_trgm ON posts USING GIN (content gin_trgm_ops);

CREATE INDEX idx_posts_created_at ON posts (created_at);

CREATE INDEX idx_posts_updated_at ON posts (updated_at);

-- Tags table
CREATE TABLE
    tags (
        id SERIAL PRIMARY KEY,
        name VARCHAR(63) UNIQUE NOT NULL
    );

CREATE INDEX idx_tags_name ON tags (name);

-- Bridge table for many-to-many relationship between posts and tags
CREATE TABLE
    post_tags (
        post_id INTEGER REFERENCES posts (id) ON DELETE CASCADE,
        tag_id INTEGER REFERENCES tags (id) ON DELETE CASCADE,
        PRIMARY KEY (post_id, tag_id)
    );

CREATE INDEX idx_post_tags_post_id ON post_tags (post_id);

CREATE INDEX idx_post_tags_tag_id ON post_tags (tag_id);