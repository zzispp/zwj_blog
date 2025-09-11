-- Your SQL goes here

CREATE TABLE snippets (
    id SERIAL PRIMARY KEY,
    title VARCHAR(191) NOT NULL UNIQUE,
    slug VARCHAR(191) NOT NULL UNIQUE,
    description VARCHAR(191) NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_snippets_published ON snippets(published);
CREATE INDEX idx_snippets_created_at ON snippets(created_at);
CREATE INDEX idx_snippets_slug ON snippets(slug);
CREATE INDEX idx_snippets_title ON snippets(title);