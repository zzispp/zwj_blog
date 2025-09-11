# API 迁移文档

本文档详细记录了从 Next.js Server Actions 到 Rust 后端 API 的迁移计划。

## 迁移状态

- ✅ **已完成**: 标签管理、文件上传
- 🔴 **待迁移**: 用户管理、博客管理、代码片段管理、笔记管理

---

## 数据库表结构

### 1. 用户表 (User)
```sql
CREATE TABLE `User` (
  `id` VARCHAR(191) NOT NULL PRIMARY KEY DEFAULT (uuid()),
  `name` VARCHAR(191),
  `password` VARCHAR(191),
  `email` VARCHAR(191) UNIQUE,
  `emailVerified` DATETIME(3),
  `image` VARCHAR(191),
  `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updatedAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
);
```

### 2. 标签表 (Tag) ✅ 已完成
```sql
CREATE TABLE `Tag` (
  `id` VARCHAR(191) NOT NULL PRIMARY KEY DEFAULT (uuid()),
  `name` VARCHAR(191) NOT NULL UNIQUE,
  `slug` VARCHAR(191) NOT NULL UNIQUE,
  `type` ENUM('ALL', 'BLOG', 'SNIPPET', 'NOTE') NOT NULL DEFAULT 'ALL',
  `icon` TEXT,
  `iconDark` TEXT,
  `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updatedAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
);
```

### 3. 博客表 (Blog)
```sql
CREATE TABLE `Blog` (
  `id` VARCHAR(191) NOT NULL PRIMARY KEY DEFAULT (uuid()),
  `title` VARCHAR(191) NOT NULL UNIQUE,
  `slug` VARCHAR(191) NOT NULL UNIQUE,
  `description` VARCHAR(191) NOT NULL,
  `body` TEXT NOT NULL,
  `cover` VARCHAR(191),
  `author` VARCHAR(191),
  `published` BOOLEAN NOT NULL DEFAULT false,
  `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updatedAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
);
```

### 4. 代码片段表 (Snippet)
```sql
CREATE TABLE `Snippet` (
  `id` VARCHAR(191) NOT NULL PRIMARY KEY DEFAULT (uuid()),
  `title` VARCHAR(191) NOT NULL UNIQUE,
  `slug` VARCHAR(191) NOT NULL UNIQUE,
  `description` VARCHAR(191) NOT NULL,
  `body` TEXT NOT NULL,
  `published` BOOLEAN NOT NULL DEFAULT false,
  `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updatedAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
);
```

### 5. 笔记表 (Note)
```sql
CREATE TABLE `Note` (
  `id` VARCHAR(191) NOT NULL PRIMARY KEY DEFAULT (uuid()),
  `body` TEXT NOT NULL,
  `published` BOOLEAN NOT NULL DEFAULT false,
  `createdAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  `updatedAt` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
);
```

### 6. 关系表 (多对多)
```sql
-- 博客-标签关系
CREATE TABLE `_BlogToTag` (
  `A` VARCHAR(191) NOT NULL REFERENCES `Blog`(`id`) ON DELETE CASCADE,
  `B` VARCHAR(191) NOT NULL REFERENCES `Tag`(`id`) ON DELETE CASCADE,
  UNIQUE INDEX `_BlogToTag_AB_unique`(`A`, `B`),
  INDEX `_BlogToTag_B_index`(`B`)
);

-- 代码片段-标签关系
CREATE TABLE `_SnippetToTag` (
  `A` VARCHAR(191) NOT NULL REFERENCES `Snippet`(`id`) ON DELETE CASCADE,
  `B` VARCHAR(191) NOT NULL REFERENCES `Tag`(`id`) ON DELETE CASCADE,
  UNIQUE INDEX `_SnippetToTag_AB_unique`(`A`, `B`),
  INDEX `_SnippetToTag_B_index`(`B`)
);

-- 笔记-标签关系
CREATE TABLE `_NoteToTag` (
  `A` VARCHAR(191) NOT NULL REFERENCES `Note`(`id`) ON DELETE CASCADE,
  `B` VARCHAR(191) NOT NULL REFERENCES `Tag`(`id`) ON DELETE CASCADE,
  UNIQUE INDEX `_NoteToTag_AB_unique`(`A`, `B`),
  INDEX `_NoteToTag_B_index`(`B`)
);
```

---

## 已完成的API

### 标签管理 ✅
| 方法 | 路径 | 功能 | 状态 |
|------|------|------|------|
| POST | /api/tags/create | 创建标签 | ✅ |
| POST | /api/tags/list | 分页获取标签列表 | ✅ |
| GET | /api/tags/all | 获取所有标签 | ✅ |
| GET | /api/tags/{id} | 获取标签详情 | ✅ |
| PUT | /api/tags/{id} | 更新标签 | ✅ |
| DELETE | /api/tags/{id} | 删除标签 | ✅ |
| GET | /api/tags/{id}/exists | 检查标签是否存在 | ✅ |

### 文件上传 ✅
| 方法 | 路径 | 功能 | 状态 |
|------|------|------|------|
| POST | /api/files/upload | 上传文件 | ✅ |

---

## 待迁移的API

### 1. 用户管理 🔴

#### 1.1 创建用户
- **路径**: `POST /api/users/create`
- **功能**: 创建新用户
- **现有实现**: `features/user/actions/index.ts` - `createUser`

**请求参数**:
```typescript
interface CreateUserRequest {
  name: string;
  email: string;
  password: string;
}
```

**响应格式**:
```typescript
interface CreateUserResponse {
  code: number;
  message: string;
  data?: {
    id: string;
    name: string;
    email: string;
    createdAt: string;
  };
}
```

#### 1.2 权限检查
- **路径**: `GET /api/users/permission`
- **功能**: 检查用户权限
- **现有实现**: `features/user/actions/index.ts` - `noPermission`

**响应格式**:
```typescript
interface PermissionResponse {
  code: number;
  message: string;
  data: {
    hasPermission: boolean;
    isAdmin: boolean;
  };
}
```

### 2. 博客管理 🔴

#### 2.1 获取博客列表
- **路径**: `POST /api/blogs/list`
- **功能**: 分页获取博客列表，支持搜索和筛选
- **现有实现**: `features/blog/actions/index.ts` - `getBlogs`

**请求参数**:
```typescript
interface GetBlogsRequest {
  limit?: number;           // 每页数量，默认10
  offset?: number;          // 偏移量，默认0
  title?: string;           // 按标题搜索
  slug?: string;            // 按slug搜索
  published?: boolean;      // 发布状态筛选
  tags?: string[];          // 标签ID数组
  orderBy?: "createdAt" | "updatedAt" | "title";
  order?: "asc" | "desc";
}
```

**响应格式**:
```typescript
interface GetBlogsResponse {
  code: number;
  message: string;
  data: {
    total: number;
    items: BlogItem[];
  };
}

interface BlogItem {
  id: string;
  title: string;
  slug: string;
  description: string;
  body: string;
  cover?: string;
  author?: string;
  published: boolean;
  tags: TagItem[];
  createdAt: string;
  updatedAt: string;
}
```

#### 2.2 获取已发布博客
- **路径**: `GET /api/blogs/published`
- **功能**: 获取所有已发布的博客
- **现有实现**: `features/blog/actions/index.ts` - `getPublishedBlogs`

#### 2.3 获取博客详情
- **路径**: `GET /api/blogs/{id}`
- **功能**: 根据ID获取博客详情
- **现有实现**: `features/blog/actions/index.ts` - `getBlogByID`

#### 2.4 根据slug获取已发布博客
- **路径**: `GET /api/blogs/slug/{slug}`
- **功能**: 根据slug获取已发布的博客（用于前台展示）
- **现有实现**: `features/blog/actions/index.ts` - `getPublishedBlogBySlug`

#### 2.5 创建博客
- **路径**: `POST /api/blogs/create`
- **功能**: 创建新博客
- **现有实现**: `features/blog/actions/index.ts` - `createBlog`

**请求参数**:
```typescript
interface CreateBlogRequest {
  title: string;
  slug: string;
  description: string;
  body: string;
  cover?: string;
  author?: string;
  published?: boolean;
  tags?: string[];          // 标签ID数组
}
```

#### 2.6 更新博客
- **路径**: `PUT /api/blogs/{id}`
- **功能**: 更新博客信息
- **现有实现**: `features/blog/actions/index.ts` - `updateBlog`

**请求参数**:
```typescript
interface UpdateBlogRequest {
  title?: string;
  slug?: string;
  description?: string;
  body?: string;
  cover?: string;
  author?: string;
  published?: boolean;
  tags?: string[];
}
```

#### 2.7 删除博客
- **路径**: `DELETE /api/blogs/{id}`
- **功能**: 删除博客
- **现有实现**: `features/blog/actions/index.ts` - `deleteBlogByID`

#### 2.8 切换博客发布状态
- **路径**: `PATCH /api/blogs/{id}/toggle-publish`
- **功能**: 切换博客的发布状态
- **现有实现**: `features/blog/actions/index.ts` - `toggleBlogPublished`

#### 2.9 检查博客是否存在
- **路径**: `GET /api/blogs/{id}/exists`
- **功能**: 检查博客是否存在
- **现有实现**: `features/blog/actions/index.ts` - `isBlogExistByID`

### 3. 代码片段管理 🔴

#### 3.1 获取代码片段列表
- **路径**: `POST /api/snippets/list`
- **功能**: 分页获取代码片段列表
- **现有实现**: `features/snippet/actions/index.ts` - `getSnippets`

**请求参数**:
```typescript
interface GetSnippetsRequest {
  limit?: number;
  offset?: number;
  title?: string;
  slug?: string;
  published?: boolean;
  tags?: string[];
  orderBy?: "createdAt" | "updatedAt" | "title";
  order?: "asc" | "desc";
}
```

**响应格式**:
```typescript
interface GetSnippetsResponse {
  code: number;
  message: string;
  data: {
    total: number;
    items: SnippetItem[];
  };
}

interface SnippetItem {
  id: string;
  title: string;
  slug: string;
  description: string;
  body: string;
  published: boolean;
  tags: TagItem[];
  createdAt: string;
  updatedAt: string;
}
```

#### 3.2 获取已发布代码片段
- **路径**: `GET /api/snippets/published`
- **现有实现**: `features/snippet/actions/index.ts` - `getPublishedSnippets`

#### 3.3 获取代码片段详情
- **路径**: `GET /api/snippets/{id}`
- **现有实现**: `features/snippet/actions/index.ts` - `getSnippetByID`

#### 3.4 根据slug获取代码片段
- **路径**: `GET /api/snippets/slug/{slug}`
- **现有实现**: `features/snippet/actions/index.ts` - `getSnippetBySlug`

#### 3.5 创建代码片段
- **路径**: `POST /api/snippets/create`
- **现有实现**: `features/snippet/actions/index.ts` - `createSnippet`

**请求参数**:
```typescript
interface CreateSnippetRequest {
  title: string;
  slug: string;
  description: string;
  body: string;
  published?: boolean;
  tags?: string[];
}
```

#### 3.6 更新代码片段
- **路径**: `PUT /api/snippets/{id}`
- **现有实现**: `features/snippet/actions/index.ts` - `updateSnippet`

#### 3.7 删除代码片段
- **路径**: `DELETE /api/snippets/{id}`
- **现有实现**: `features/snippet/actions/index.ts` - `deleteSnippetByID`

#### 3.8 切换代码片段发布状态
- **路径**: `PATCH /api/snippets/{id}/toggle-publish`
- **现有实现**: `features/snippet/actions/index.ts` - `toggleSnippetPublished`

#### 3.9 检查代码片段是否存在
- **路径**: `GET /api/snippets/{id}/exists`
- **现有实现**: `features/snippet/actions/index.ts` - `isSnippetExistByID`

### 4. 笔记管理 🔴

#### 4.1 获取笔记列表
- **路径**: `POST /api/notes/list`
- **功能**: 分页获取笔记列表
- **现有实现**: `features/note/actions/index.ts` - `getNotes`

**请求参数**:
```typescript
interface GetNotesRequest {
  limit?: number;
  offset?: number;
  body?: string;           // 按内容搜索
  published?: boolean;
  tags?: string[];
  orderBy?: "createdAt" | "updatedAt";
  order?: "asc" | "desc";
}
```

**响应格式**:
```typescript
interface GetNotesResponse {
  code: number;
  message: string;
  data: {
    total: number;
    items: NoteItem[];
  };
}

interface NoteItem {
  id: string;
  body: string;
  published: boolean;
  tags: TagItem[];
  createdAt: string;
  updatedAt: string;
}
```

#### 4.2 获取所有笔记
- **路径**: `GET /api/notes/all`
- **现有实现**: `features/note/actions/index.ts` - `getAllNotes`

#### 4.3 获取笔记详情
- **路径**: `GET /api/notes/{id}`
- **现有实现**: `features/note/actions/index.ts` - `getNoteByID`

#### 4.4 创建笔记
- **路径**: `POST /api/notes/create`
- **现有实现**: `features/note/actions/index.ts` - `createNote`

**请求参数**:
```typescript
interface CreateNoteRequest {
  body: string;
  published?: boolean;
  tags?: string[];
}
```

#### 4.5 更新笔记
- **路径**: `PUT /api/notes/{id}`
- **现有实现**: `features/note/actions/index.ts` - `updateNote`

#### 4.6 删除笔记
- **路径**: `DELETE /api/notes/{id}`
- **现有实现**: `features/note/actions/index.ts` - `deleteNoteByID`

#### 4.7 切换笔记发布状态
- **路径**: `PATCH /api/notes/{id}/toggle-publish`
- **现有实现**: `features/note/actions/index.ts` - `toggleNotePublished`

#### 4.8 检查笔记是否存在
- **路径**: `GET /api/notes/{id}/exists`
- **现有实现**: `features/note/actions/index.ts` - `isNoteExistByID`

---

## 通用响应格式

所有API都使用统一的响应格式：

```typescript
interface ApiResponse<T> {
  code: number;        // 200: 成功, 400: 客户端错误, 500: 服务器错误
  message: string;     // 响应消息
  data?: T;           // 响应数据（可选）
}
```

### 错误响应示例
```json
{
  "code": 400,
  "message": "请求参数错误: title不能为空"
}
```

### 成功响应示例
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "total": 25,
    "items": [...]
  }
}
```

---

## 分页说明

所有列表类API都支持分页：

- `limit`: 每页返回的记录数，默认10条，最大100条
- `offset`: 跳过的记录数，用于分页计算
- `total`: 符合条件的总记录数

### 分页计算示例
- 第1页: `offset: 0, limit: 10`
- 第2页: `offset: 10, limit: 10`  
- 第3页: `offset: 20, limit: 10`

---

## 类型映射

### 标签类型映射
前端使用的枚举值与后端需要保持一致：

```typescript
// 前端 -> 后端
const typeMapping = {
  "ALL": "All",
  "BLOG": "Blog", 
  "NOTE": "Note",
  "SNIPPET": "Snippet"
};
```

### 发布状态映射
```typescript
// 前端 -> 后端
const publishedMapping = {
  "all": undefined,        // 查询所有
  "published": true,       // 仅已发布
  "draft": false          // 仅草稿
};
```

---

## 迁移优先级

建议按以下优先级进行迁移：

1. **🔥 用户管理** - 基础认证功能
2. **📝 博客管理** - 核心内容管理功能 
3. **💻 代码片段管理** - 开发相关内容
4. **📋 笔记管理** - 简单的内容管理

每个模块迁移完成后，需要更新对应的前端API调用层和组件。

---

## 注意事项

1. **ID格式**: 所有ID都使用字符串格式（cuid）
2. **时间格式**: 使用ISO 8601格式的字符串
3. **文本字段**: `body`字段使用TEXT类型，支持大文本内容
4. **唯一约束**: `title`和`slug`都有唯一约束
5. **级联删除**: 删除内容时需要同时处理关联的标签关系
6. **权限控制**: 所有管理类API都需要验证用户权限
7. **数据验证**: 后端需要对所有输入参数进行验证
8. **事务处理**: 涉及多表操作的API需要使用数据库事务

---

## 前端迁移checklist

每个API迁移时需要完成：

- [ ] 创建对应的API调用函数 (`lib/{module}-api.ts`)
- [ ] 更新相关的hooks (`features/{module}/api/`)
- [ ] 移除对应的server actions
- [ ] 更新类型定义
- [ ] 测试功能完整性
- [ ] 处理错误状态和加载状态
