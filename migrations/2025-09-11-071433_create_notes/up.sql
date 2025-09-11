-- Your SQL goes here

CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_notes_published ON notes(published);
CREATE INDEX idx_notes_created_at ON notes(created_at);