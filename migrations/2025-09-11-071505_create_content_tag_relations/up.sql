-- Your SQL goes here

-- 博客-标签关系表
CREATE TABLE blog_tag_relations (
    blog_id INTEGER NOT NULL REFERENCES blogs(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (blog_id, tag_id)
);

-- 笔记-标签关系表
CREATE TABLE note_tag_relations (
    note_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

-- 代码片段-标签关系表
CREATE TABLE snippet_tag_relations (
    snippet_id INTEGER NOT NULL REFERENCES snippets(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (snippet_id, tag_id)
);

-- Create indexes for better performance
CREATE INDEX idx_blog_tag_relations_blog_id ON blog_tag_relations(blog_id);
CREATE INDEX idx_blog_tag_relations_tag_id ON blog_tag_relations(tag_id);
CREATE INDEX idx_note_tag_relations_note_id ON note_tag_relations(note_id);
CREATE INDEX idx_note_tag_relations_tag_id ON note_tag_relations(tag_id);
CREATE INDEX idx_snippet_tag_relations_snippet_id ON snippet_tag_relations(snippet_id);
CREATE INDEX idx_snippet_tag_relations_tag_id ON snippet_tag_relations(tag_id);