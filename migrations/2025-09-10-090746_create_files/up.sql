CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    file_hash VARCHAR(32) NOT NULL UNIQUE, -- MD5 hash作为唯一索引
    file_path VARCHAR NOT NULL, -- 文件位置，不带http host
    upload_time TIMESTAMP NOT NULL DEFAULT NOW(), -- 文件上传时间
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 为hash创建唯一索引以提高查询性能
CREATE UNIQUE INDEX idx_files_hash ON files(file_hash);

-- 设置自动更新时间戳
SELECT diesel_manage_updated_at('files');