-- Your SQL goes here

CREATE TABLE blogs (
    id SERIAL PRIMARY KEY,
    title VARCHAR(191) NOT NULL UNIQUE,
    slug VARCHAR(191) NOT NULL UNIQUE,
    description VARCHAR(191) NOT NULL,
    body TEXT NOT NULL,
    cover VARCHAR(191),
    author VARCHAR(191),
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_blogs_published ON blogs(published);
CREATE INDEX idx_blogs_created_at ON blogs(created_at);
CREATE INDEX idx_blogs_slug ON blogs(slug);
CREATE INDEX idx_blogs_title ON blogs(title);