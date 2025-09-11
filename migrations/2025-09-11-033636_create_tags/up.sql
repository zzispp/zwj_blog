-- Your SQL goes here

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(191) NOT NULL UNIQUE,
    slug VARCHAR(191) NOT NULL UNIQUE,
    type VARCHAR(20) NOT NULL DEFAULT 'ALL' CHECK (type IN ('ALL', 'BLOG', 'SNIPPET', 'NOTE')),
    icon TEXT,
    icon_dark TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_tags_type ON tags(type);
CREATE INDEX idx_tags_created_at ON tags(created_at);
