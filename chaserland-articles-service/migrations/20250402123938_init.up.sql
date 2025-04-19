-- Add up migration script here

-- create article schema
CREATE SCHEMA IF NOT EXISTS article;

-- create article series and add index on slug
CREATE TABLE IF NOT EXISTS article.series(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE
);
CREATE UNIQUE INDEX IF NOT EXISTS article_series_slug_idx on article.series (slug);

-- create article tables and add index on slug
CREATE TABLE IF NOT EXISTS article.articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    content TEXT,
    series_id INT REFERENCES article.series(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);
CREATE UNIQUE INDEX IF NOT EXISTS article_articles_slug_idx on article.articles (slug);
CREATE INDEX IF NOT EXISTS article_articles_published_at_idx on article.articles (published_at);
CREATE INDEX IF NOT EXISTS article_articles_deleted_at_idx on article.articles (deleted_at);
CREATE OR REPLACE VIEW article.articles_meta AS SELECT id, title, slug, description, series_id, created_at, published_at, updated_at, deleted_at FROM article.articles;
CREATE OR REPLACE VIEW article.public_articles AS SELECT * FROM article.articles WHERE published_at IS NOT NULL AND deleted_at IS NULL;
CREATE OR REPLACE VIEW article.public_articles_meta AS SELECT id, title, slug, description, series_id, created_at, published_at, updated_at, deleted_at FROM article.public_articles;

-- create article tags and add index on slug
CREATE TABLE IF NOT EXISTS article.tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE
);
CREATE UNIQUE INDEX IF NOT EXISTS article_tags_slug_idx on article.tags (slug);

-- create article categories and add index on slug
CREATE TABLE IF NOT EXISTS article.categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE
);
CREATE UNIQUE INDEX IF NOT EXISTS article_categories_slug_idx on article.categories (slug);

-- create article-tag relation table
CREATE TABLE IF NOT EXISTS article.article_tags (
    article_id INT NOT NULL REFERENCES article.articles(id) ON DELETE CASCADE,
    tag_id INT NOT NULL REFERENCES article.tags(id) ON DELETE CASCADE,
    PRIMARY KEY (article_id, tag_id)
);

-- create article-category relation table
CREATE TABLE IF NOT EXISTS article.article_categories (
    article_id INT NOT NULL REFERENCES article.articles(id) ON DELETE CASCADE,  
    category_id INT NOT NULL REFERENCES article.categories(id) ON DELETE CASCADE,
    PRIMARY KEY (article_id, category_id)
);
