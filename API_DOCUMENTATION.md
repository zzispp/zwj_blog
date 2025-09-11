# Blog API 文档

## 基本信息

- **服务器地址**: `http://localhost:8080`
- **API基础路径**: `/api`
- **内容类型**: `application/json`
- **字符编码**: `UTF-8`

## 统一响应格式

所有API接口都遵循统一的响应格式：

```json
{
  "code": 200,           // 状态码：200成功，500错误
  "message": "success",  // 响应消息
  "data": {}            // 响应数据
}
```

## 分页响应格式

对于列表接口，响应数据格式为：

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "total": 100,        // 总记录数
    "items": []          // 数据列表
  }
}
```

---

## 1. 标签管理 (Tags)

### 1.1 创建标签

**接口地址**: `POST /api/tags/create`

**请求参数**:
```json
{
  "name": "前端技术",           // 标签名称，必填
  "slug": "frontend",         // URL友好的标识符，必填，唯一
  "type": "BLOG",            // 标签类型：ALL|BLOG|NOTE|SNIPPET，必填
  "icon": "icon-url",        // 图标URL，可选
  "iconDark": "dark-icon-url" // 深色主题图标URL，可选
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "name": "前端技术",
    "slug": "frontend",
    "type": "BLOG",
    "icon": "icon-url",
    "iconDark": "dark-icon-url",
    "created_at": "2025-09-11T07:00:00Z",
    "updated_at": "2025-09-11T07:00:00Z"
  }
}
```

### 1.2 获取标签列表

**接口地址**: `POST /api/tags/list`

**请求参数**:
```json
{
  "limit": 10,           // 每页数量，可选，默认10
  "offset": 0,           // 偏移量，可选，默认0
  "name": "前端",         // 按名称筛选，可选
  "slug": "frontend",    // 按slug筛选，可选
  "type": "BLOG",        // 按类型筛选，可选
  "orderBy": "created_at", // 排序字段，可选
  "order": "desc"        // 排序方向：asc|desc，可选
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "total": 1,
    "items": [
      {
        "id": 1,
        "name": "前端技术",
        "slug": "frontend",
        "type": "BLOG",
        "icon": "icon-url",
        "iconDark": "dark-icon-url",
        "created_at": "2025-09-11T07:00:00Z",
        "updated_at": "2025-09-11T07:00:00Z"
      }
    ]
  }
}
```

### 1.3 获取标签详情

**接口地址**: `GET /api/tags/{id}`

**路径参数**:
- `id`: 标签ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "name": "前端技术",
    "slug": "frontend",
    "type": "BLOG",
    "icon": "icon-url",
    "iconDark": "dark-icon-url",
    "created_at": "2025-09-11T07:00:00Z",
    "updated_at": "2025-09-11T07:00:00Z"
  }
}
```

### 1.4 更新标签

**接口地址**: `PUT /api/tags/{id}`

**路径参数**:
- `id`: 标签ID

**请求参数**:
```json
{
  "name": "前端开发",         // 可选
  "slug": "frontend-dev",    // 可选
  "type": "BLOG",           // 可选
  "icon": "new-icon-url",   // 可选
  "iconDark": "new-dark-icon-url" // 可选
}
```

**响应示例**: 同获取标签详情

### 1.5 删除标签

**接口地址**: `DELETE /api/tags/{id}`

**路径参数**:
- `id`: 标签ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "success": true
  }
}
```

### 1.6 检查标签是否存在

**接口地址**: `GET /api/tags/{id}/exists`

**路径参数**:
- `id`: 标签ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "exists": true
  }
}
```

### 1.7 获取所有标签

**接口地址**: `GET /api/tags/all`

**查询参数**:
- `type`: 标签类型筛选，可选 (ALL|BLOG|NOTE|SNIPPET)

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "tags": [
      {
        "id": 1,
        "name": "前端技术",
        "slug": "frontend",
        "type": "BLOG",
        "icon": "icon-url",
        "iconDark": "dark-icon-url",
        "created_at": "2025-09-11T07:00:00Z",
        "updated_at": "2025-09-11T07:00:00Z"
      }
    ],
    "total": 1
  }
}
```

---

## 2. 博客管理 (Blogs)

### 2.1 创建博客

**接口地址**: `POST /api/blogs/create`

**请求参数**:
```json
{
  "title": "我的第一篇博客",        // 标题，必填，唯一
  "slug": "my-first-blog",       // URL友好标识符，必填，唯一
  "description": "这是一篇关于...", // 描述，必填
  "body": "博客正文内容...",      // 正文，必填
  "cover": "cover-image-url",    // 封面图片URL，可选
  "author": "张三",              // 作者，可选
  "published": true,             // 是否发布，可选，默认false
  "tags": [1, 2, 3]             // 标签ID数组，可选
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "title": "我的第一篇博客",
    "slug": "my-first-blog",
    "description": "这是一篇关于...",
    "body": "博客正文内容...",
    "cover": "cover-image-url",
    "author": "张三",
    "published": true,
    "tags": [
      {
        "id": 1,
        "name": "前端技术",
        "slug": "frontend",
        "type": "BLOG",
        "icon": "icon-url",
        "iconDark": "dark-icon-url",
        "created_at": "2025-09-11T07:00:00Z",
        "updated_at": "2025-09-11T07:00:00Z"
      }
    ],
    "created_at": "2025-09-11T07:00:00Z",
    "updated_at": "2025-09-11T07:00:00Z"
  }
}
```

### 2.2 获取博客列表

**接口地址**: `POST /api/blogs/list`

**请求参数**:
```json
{
  "limit": 10,              // 每页数量，可选，默认10
  "offset": 0,              // 偏移量，可选，默认0
  "title": "博客",          // 按标题筛选，可选
  "slug": "my-blog",        // 按slug筛选，可选
  "author": "张三",         // 按作者筛选，可选
  "published": true,        // 按发布状态筛选，可选
  "tags": [1, 2],          // 按标签筛选，可选
  "orderBy": "created_at",  // 排序字段，可选
  "order": "desc"          // 排序方向，可选
}
```

**响应示例**: 分页格式，data.items为博客数组

### 2.3 获取博客详情

**接口地址**: `GET /api/blogs/{id}`

**路径参数**:
- `id`: 博客ID

**响应示例**: 同创建博客响应

### 2.4 根据Slug获取博客

**接口地址**: `GET /api/blogs/slug/{slug}`

**路径参数**:
- `slug`: 博客slug

**响应示例**: 同创建博客响应

### 2.5 获取已发布博客

**接口地址**: `GET /api/blogs/published`

**响应示例**: 已发布博客数组

### 2.6 根据Slug获取已发布博客

**接口地址**: `GET /api/blogs/published/slug/{slug}`

**路径参数**:
- `slug`: 博客slug

**响应示例**: 同创建博客响应

### 2.7 更新博客

**接口地址**: `PUT /api/blogs/{id}`

**路径参数**:
- `id`: 博客ID

**请求参数**: 所有字段都是可选的
```json
{
  "title": "更新的标题",
  "slug": "updated-slug",
  "description": "更新的描述",
  "body": "更新的正文",
  "cover": "new-cover-url",
  "author": "新作者",
  "published": false,
  "tags": [2, 3, 4]
}
```

**响应示例**: 同创建博客响应

### 2.8 删除博客

**接口地址**: `DELETE /api/blogs/{id}`

**路径参数**:
- `id`: 博客ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "success": true
  }
}
```

### 2.9 检查博客是否存在

**接口地址**: `GET /api/blogs/{id}/exists`

**路径参数**:
- `id`: 博客ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "exists": true
  }
}
```

### 2.10 切换博客发布状态

**接口地址**: `PATCH /api/blogs/{id}/toggle-publish`

**路径参数**:
- `id`: 博客ID

**响应示例**: 同创建博客响应

---

## 3. 笔记管理 (Notes)

### 3.1 创建笔记

**接口地址**: `POST /api/notes/create`

**请求参数**:
```json
{
  "body": "笔记内容...",      // 笔记正文，必填
  "published": true,        // 是否发布，可选，默认false
  "tags": [1, 2, 3]        // 标签ID数组，可选
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "body": "笔记内容...",
    "published": true,
    "tags": [
      {
        "id": 1,
        "name": "学习笔记",
        "slug": "study-note",
        "type": "NOTE",
        "icon": "note-icon",
        "iconDark": "note-dark-icon",
        "created_at": "2025-09-11T07:00:00Z",
        "updated_at": "2025-09-11T07:00:00Z"
      }
    ],
    "created_at": "2025-09-11T07:00:00Z",
    "updated_at": "2025-09-11T07:00:00Z"
  }
}
```

### 3.2 获取笔记列表

**接口地址**: `POST /api/notes/list`

**请求参数**:
```json
{
  "limit": 10,              // 每页数量，可选，默认10
  "offset": 0,              // 偏移量，可选，默认0
  "body": "关键词",         // 按内容筛选，可选
  "published": true,        // 按发布状态筛选，可选
  "tags": [1, 2],          // 按标签筛选，可选
  "orderBy": "created_at",  // 排序字段，可选
  "order": "desc"          // 排序方向，可选
}
```

**响应示例**: 分页格式，data.items为笔记数组

### 3.3 获取笔记详情

**接口地址**: `GET /api/notes/{id}`

**路径参数**:
- `id`: 笔记ID

**响应示例**: 同创建笔记响应

### 3.4 获取所有笔记

**接口地址**: `GET /api/notes/all`

**响应示例**: 所有笔记数组

### 3.5 更新笔记

**接口地址**: `PUT /api/notes/{id}`

**路径参数**:
- `id`: 笔记ID

**请求参数**: 所有字段都是可选的
```json
{
  "body": "更新的笔记内容",
  "published": false,
  "tags": [2, 3, 4]
}
```

**响应示例**: 同创建笔记响应

### 3.6 删除笔记

**接口地址**: `DELETE /api/notes/{id}`

**路径参数**:
- `id`: 笔记ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "success": true
  }
}
```

### 3.7 检查笔记是否存在

**接口地址**: `GET /api/notes/{id}/exists`

**路径参数**:
- `id`: 笔记ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "exists": true
  }
}
```

### 3.8 切换笔记发布状态

**接口地址**: `PATCH /api/notes/{id}/toggle-publish`

**路径参数**:
- `id`: 笔记ID

**响应示例**: 同创建笔记响应

---

## 4. 代码片段管理 (Snippets)

### 4.1 创建代码片段

**接口地址**: `POST /api/snippets/create`

**请求参数**:
```json
{
  "title": "快速排序算法",           // 标题，必填，唯一
  "slug": "quick-sort",           // URL友好标识符，必填，唯一
  "description": "JavaScript实现的快速排序", // 描述，必填
  "body": "function quickSort(arr) {...}", // 代码内容，必填
  "published": true,              // 是否发布，可选，默认false
  "tags": [1, 2, 3]              // 标签ID数组，可选
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "title": "快速排序算法",
    "slug": "quick-sort",
    "description": "JavaScript实现的快速排序",
    "body": "function quickSort(arr) {...}",
    "published": true,
    "tags": [
      {
        "id": 1,
        "name": "算法",
        "slug": "algorithm",
        "type": "SNIPPET",
        "icon": "algo-icon",
        "iconDark": "algo-dark-icon",
        "created_at": "2025-09-11T07:00:00Z",
        "updated_at": "2025-09-11T07:00:00Z"
      }
    ],
    "created_at": "2025-09-11T07:00:00Z",
    "updated_at": "2025-09-11T07:00:00Z"
  }
}
```

### 4.2 获取代码片段列表

**接口地址**: `POST /api/snippets/list`

**请求参数**:
```json
{
  "limit": 10,              // 每页数量，可选，默认10
  "offset": 0,              // 偏移量，可选，默认0
  "title": "排序",          // 按标题筛选，可选
  "slug": "sort",           // 按slug筛选，可选
  "published": true,        // 按发布状态筛选，可选
  "tags": [1, 2],          // 按标签筛选，可选
  "orderBy": "created_at",  // 排序字段，可选
  "order": "desc"          // 排序方向，可选
}
```

**响应示例**: 分页格式，data.items为代码片段数组

### 4.3 获取代码片段详情

**接口地址**: `GET /api/snippets/{id}`

**路径参数**:
- `id`: 代码片段ID

**响应示例**: 同创建代码片段响应

### 4.4 根据Slug获取代码片段

**接口地址**: `GET /api/snippets/slug/{slug}`

**路径参数**:
- `slug`: 代码片段slug

**响应示例**: 同创建代码片段响应

### 4.5 获取已发布代码片段

**接口地址**: `GET /api/snippets/published`

**响应示例**: 已发布代码片段数组

### 4.6 更新代码片段

**接口地址**: `PUT /api/snippets/{id}`

**路径参数**:
- `id`: 代码片段ID

**请求参数**: 所有字段都是可选的
```json
{
  "title": "更新的标题",
  "slug": "updated-slug",
  "description": "更新的描述",
  "body": "更新的代码",
  "published": false,
  "tags": [2, 3, 4]
}
```

**响应示例**: 同创建代码片段响应

### 4.7 删除代码片段

**接口地址**: `DELETE /api/snippets/{id}`

**路径参数**:
- `id`: 代码片段ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "success": true
  }
}
```

### 4.8 检查代码片段是否存在

**接口地址**: `GET /api/snippets/{id}/exists`

**路径参数**:
- `id`: 代码片段ID

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "exists": true
  }
}
```

### 4.9 切换代码片段发布状态

**接口地址**: `PATCH /api/snippets/{id}/toggle-publish`

**路径参数**:
- `id`: 代码片段ID

**响应示例**: 同创建代码片段响应

---

## 5. 文件管理 (Files)

### 5.1 上传文件

**接口地址**: `POST /api/files/upload`

**请求参数**: 
- 使用 `multipart/form-data` 格式
- 字段名: `file`

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "filename": "image.jpg",
    "content_type": "image/jpeg",
    "size": 1024000,
    "url": "/static/abc123.jpg",
    "created_at": "2025-09-11T07:00:00Z"
  }
}
```

---

## 6. 待办事项 (Todos)

### 6.1 创建待办事项

**接口地址**: `POST /api/todos/create`

**请求参数**:
```json
{
  "title": "完成API文档",
  "completed": false
}
```

### 6.2 获取待办事项列表

**接口地址**: `GET /api/todos`

### 6.3 更新待办事项

**接口地址**: `PUT /api/todos/{id}`

### 6.4 删除待办事项

**接口地址**: `DELETE /api/todos/{id}`

---

## 7. JWT认证

### 7.1 认证说明

所有的**创建、更新、删除**操作都需要JWT认证，包括：
- `POST` 请求包含 `/create` 路径
- `PUT` 请求（更新操作）
- `DELETE` 请求（删除操作）
- `PATCH` 请求（部分更新操作）

**不需要认证的操作**：
- `GET` 请求（查询操作）
- `POST` 请求的 `/list` 路径（分页查询）

### 7.2 如何携带JWT Token

在请求头中添加Authorization字段：

```bash
Authorization: Bearer YOUR_JWT_TOKEN_HERE
```

### 7.3 获取JWT Token

**接口地址**: `POST /api/users/verify-signature`

**请求参数**:
```json
{
  "address": "用户钱包地址",
  "signature": "签名字符串"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "address": "用户钱包地址"
  }
}
```

### 7.4 JWT认证错误响应

#### 7.4.1 缺少Authorization头

**HTTP状态码**: `401 Unauthorized`

**响应内容**:
```
Missing authorization header
```

#### 7.4.2 Authorization格式错误

**HTTP状态码**: `401 Unauthorized`

**响应内容**:
```
Authorization header must start with 'Bearer '
```

#### 7.4.3 Token无效或过期

**HTTP状态码**: `401 Unauthorized`

**响应内容**:
```
Invalid or expired token
```

#### 7.4.4 Authorization头编码错误

**HTTP状态码**: `401 Unauthorized`

**响应内容**:
```
Invalid authorization header format
```

---

## 错误响应

当请求出错时，返回格式如下：

```json
{
  "code": 500,
  "message": "错误信息描述",
  "data": null
}
```

常见状态码：
- `200`: 请求成功
- `401`: 未授权（JWT认证失败）
- `404`: 资源不存在
- `500`: 服务器内部错误

---

## 使用示例

### JWT认证流程示例

#### 1. 获取nonce
```bash
curl -X POST http://localhost:8080/api/users/get-nonce \
  -H "Content-Type: application/json" \
  -d '{
    "address": "YOUR_WALLET_ADDRESS"
  }'
```

#### 2. 验证签名获取JWT Token
```bash
curl -X POST http://localhost:8080/api/users/verify-signature \
  -H "Content-Type: application/json" \
  -d '{
    "address": "YOUR_WALLET_ADDRESS",
    "signature": "YOUR_SIGNATURE"
  }'
```

#### 3. 使用JWT Token创建博客

```bash
curl -X POST http://localhost:8080/api/blogs/create \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE" \
  -d '{
    "title": "我的第一篇博客",
    "slug": "my-first-blog",
    "description": "这是一篇关于前端开发的博客",
    "body": "# 前端开发入门\n\n这里是正文内容...",
    "author": "张三",
    "published": true,
    "tags": [1, 2]
  }'
```

### 无需认证的查询示例

### 获取博客列表示例

```bash
curl -X POST http://localhost:8080/api/blogs/list \
  -H "Content-Type: application/json" \
  -d '{
    "limit": 10,
    "offset": 0,
    "published": true
  }'
```

### 上传文件示例

```bash
curl -X POST http://localhost:8080/api/files/upload \
  -F "file=@/path/to/your/image.jpg"
```

---

## 注意事项

1. 所有POST请求都需要设置 `Content-Type: application/json`
2. 文件上传接口使用 `multipart/form-data` 格式
3. 时间格式统一使用 ISO 8601 格式 (RFC 3339)
4. 所有文本字段都支持UTF-8编码
5. slug字段必须是URL友好的格式（小写字母、数字、连字符）
6. 标签关系会自动处理，无需手动管理关联表
7. 删除操作支持级联删除（删除内容时会自动删除相关的标签关联）
8. **JWT认证**：创建、更新、删除操作需要在请求头中携带有效的JWT token
9. **Token有效期**：JWT token有效期为2小时，过期后需要重新获取
10. **错误处理**：前端应监听401状态码，提示用户重新登录

---

## 开发信息

- **技术栈**: Rust + Actix Web + Diesel + PostgreSQL
- **架构模式**: 分层架构（Domain-Repository-Service-Handler）
- **ORM**: Diesel 2.x
- **数据库**: PostgreSQL
- **认证方式**: JWT Token (HS256算法)
- **密钥管理**: 文件存储 (`jwt_key.bin`)
- **日志**: tracing
- **序列化**: serde_json
