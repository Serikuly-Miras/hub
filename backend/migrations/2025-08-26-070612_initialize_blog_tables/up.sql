-- Create the pg_trgm extension
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Blog statuses table
CREATE TABLE
    blog_statuses (
        id SERIAL PRIMARY KEY,
        name VARCHAR(63) UNIQUE NOT NULL
    );

CREATE INDEX idx_blog_statuses_name ON blog_statuses (name);

-- Blogs table
CREATE TABLE
    blogs (
        id SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        status INTEGER REFERENCES blog_statuses (id),
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT NOW (),
        updated_at TIMESTAMP DEFAULT NOW (),
        deleted_at TIMESTAMP
    );

CREATE INDEX idx_blogs_title_trgm ON blogs USING GIN (title gin_trgm_ops);

CREATE INDEX idx_blogs_content_trgm ON blogs USING GIN (content gin_trgm_ops);

CREATE INDEX idx_blogs_created_at ON blogs (created_at);

CREATE INDEX idx_blogs_updated_at ON blogs (updated_at);

-- Tags table
CREATE TABLE
    tags (
        id SERIAL PRIMARY KEY,
        name VARCHAR(63) UNIQUE NOT NULL
    );

CREATE INDEX idx_tags_name ON tags (name);

-- Bridge table for many-to-many relationship between blogs and tags
CREATE TABLE
    blog_tags (
        blog_id INTEGER REFERENCES blogs (id) ON DELETE CASCADE,
        tag_id INTEGER REFERENCES tags (id) ON DELETE CASCADE,
        PRIMARY KEY (blog_id, tag_id)
    );

CREATE INDEX idx_blog_tags_blog_id ON blog_tags (blog_id);

CREATE INDEX idx_blog_tags_tag_id ON blog_tags (tag_id);