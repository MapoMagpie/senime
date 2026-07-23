#!/usr/bin/env python3
"""从 gemoji JSON 生成 senime FuzzDict 用的 emoji 码表。

数据来源: https://github.com/github/gemoji
输出格式: emoji<Tab>描述;标签,们;别名,们

用法:
    curl -sL https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json \
        | python3 scripts/generate-emoji-dict.py > emoji.txt
"""

import json
import re
import sys


def main():
    data = json.load(sys.stdin)

    lines = []
    for entry in data:
        emoji = entry["emoji"]
        desc = entry["description"].strip()

        tags = entry.get("tags", [])
        aliases = entry.get("aliases", [])

        # 将 description 拆成单词集合，用于去重 aliases
        desc_words = set(re.findall(r"[a-z0-9]+", desc.lower()))

        # 去重：alias 的所有单词都被 description 覆盖 → 跳过
        deduped_aliases = []
        for a in aliases:
            a_lower = a.lower().strip()
            a_words = set(re.findall(r"[a-z0-9]+", a_lower))
            if a_words and a_words.issubset(desc_words):
                continue
            deduped_aliases.append(a)

        # 构建 code: 去掉尾部空段，避免 ;; 或尾部 ;
        segments = [desc]
        if tags:
            segments.append(",".join(tags))
        if deduped_aliases:
            segments.append(",".join(deduped_aliases))
        code = ";".join(segments)

        lines.append(f"{emoji}\t{code}")

    for line in lines:
        print(line)


if __name__ == "__main__":
    main()
