#!/bin/bash

# 创建基础目录
BASE_DIR="articles"
mkdir -p $BASE_DIR

# 示例标签和关键词池
TAGS=("技术" "编程" "教程" "开发" "软件" "云计算" "人工智能" "数据库" "前端" "后端")
KEYWORDS=("示例" "文档" "指南" "实践" "入门" "进阶" "原理" "架构" "设计" "优化")

# 文章主题池
TOPICS=("Python编程" "JavaScript开发" "云原生技术" "DevOps实践" "微服务架构" "数据库优化" "前端框架" "后端开发" "系统设计" "性能优化")

# 段落内容池
PARAGRAPHS=(
    "在软件开发领域，代码质量和可维护性始终是开发者需要关注的重点。通过采用合适的设计模式和架构原则，我们可以构建出更加健壮和可扩展的系统。"
    "持续集成和持续部署（CI/CD）已经成为现代软件开发流程中不可或缺的一部分。通过自动化构建、测试和部署流程，团队可以更快速地交付高质量的软件。"
    "微服务架构的兴起带来了新的机遇和挑战。虽然它提供了更好的可扩展性和部署灵活性，但同时也增加了系统的复杂性和运维难度。"
    "容器技术的普及极大地改变了应用程序的部署方式。Docker和Kubernetes等工具使得应用程序的打包、分发和运行变得更加标准化和简单。"
    "在数据密集型应用程序中，数据库性能优化是一个永恒的话题。从索引优化到查询改写，从缓存策略到分库分表，都需要深入的理解和实践。"
)

# 代码示例池
CODE_EXAMPLES=(
    "```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
```"

    "```javascript
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}
```"

    "```sql
SELECT 
    users.name,
    COUNT(orders.id) as order_count
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id
HAVING order_count > 5;
```"
)

# 生成随机日期（2020-2024年间）
generate_random_date() {
    year=$(shuf -i 2020-2024 -n 1)
    month=$(shuf -i 1-12 -n 1)
    day=$(shuf -i 1-28 -n 1)
    printf "%04d%02d%02d" $year $month $day
}

# 生成随机标签
generate_random_tags() {
    local num_tags=$(shuf -i 2-5 -n 1)
    local tags=($(shuf -e "${TAGS[@]}" | head -n $num_tags))
    local result=""
    for tag in "${tags[@]}"; do
        result+="\"$tag\", "
    done
    echo "${result%, }"
}

# 生成随机关键词
generate_random_keywords() {
    local num_keywords=$(shuf -i 3-6 -n 1)
    local keywords=($(shuf -e "${KEYWORDS[@]}" | head -n $num_keywords))
    local result=""
    for keyword in "${keywords[@]}"; do
        result+="\"$keyword\", "
    done
    echo "${result%, }"
}

# 生成随机段落
generate_random_paragraph() {
    echo "${PARAGRAPHS[RANDOM % ${#PARAGRAPHS[@]}]}"
}

# 生成随机代码示例
generate_random_code() {
    echo "${CODE_EXAMPLES[RANDOM % ${#CODE_EXAMPLES[@]}]}"
}

# 生成1000篇文章
for id in {1..1000}; do
    # 创建文章目录
    article_dir="$BASE_DIR/$id"
    mkdir -p "$article_dir"
    
    # 随机选择一个主题
    topic=${TOPICS[RANDOM % ${#TOPICS[@]}]}
    
    # 生成metainfo.toml
    cat > "$article_dir/metainfo.toml" << EOF
[article]
id = $id
title = "${topic}详解 #$id"
description = "这是一篇关于${topic}的深入解析文章，涵盖了理论基础和实践应用。"
markdown_path = "content.md"
date = $(generate_random_date)
tags = [$(generate_random_tags)]
keywords = [$(generate_random_keywords)]
EOF

    # 生成content.md
    cat > "$article_dir/content.md" << EOF
# ${topic}详解 #$id

## 引言

$(generate_random_paragraph)

## 基础概念

$(generate_random_paragraph)
$(generate_random_paragraph)

### 核心要点

$(generate_random_paragraph)

## 实践应用

$(generate_random_paragraph)

### 代码示例

$(generate_random_code)

### 最佳实践

$(generate_random_paragraph)
$(generate_random_paragraph)

## 进阶技巧

$(generate_random_paragraph)

### 性能优化

$(generate_random_paragraph)

### 常见问题解决

$(generate_random_paragraph)
$(generate_random_code)

## 实际案例分析

$(generate_random_paragraph)
$(generate_random_paragraph)

### 案例一

$(generate_random_paragraph)

### 案例二

$(generate_random_paragraph)

## 总结

$(generate_random_paragraph)

## 参考资料

1. 《${topic}权威指南》
2. ${topic}官方文档
3. 业界最佳实践参考
EOF

    echo "已生成文章 $id"
done

echo "完成！共生成了1000篇测试文章。"

