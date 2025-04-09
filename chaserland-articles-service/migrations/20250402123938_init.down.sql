-- Add down migration script here

DROP TABLE IF EXISTS article.article_categories;

DROP TABLE IF EXISTS article.article_tags;

DROP INDEX IF EXISTS article_categories_slug_idx;
DROP TABLE IF EXISTS article.categories;

DROP INDEX IF EXISTS article_tags_slug_idx;
DROP TABLE IF EXISTS article.tags;

DROP VIEW IF EXISTS article.public_articles_meta;
DROP VIEW IF EXISTS article.public_articles;
DROP VIEW IF EXISTS article.articles_meta;
DROP INDEX IF EXISTS article_articles_slug_idx;
DROP INDEX IF EXISTS article_articles_published_at_idx;
DROP INDEX IF EXISTS article_articles_deleted_at_idx;
DROP TABLE IF EXISTS article.articles;

DROP INDEX IF EXISTS article_series_slug_idx;
DROP TABLE IF EXISTS article.series;

DROP SCHEMA IF EXISTS article;
