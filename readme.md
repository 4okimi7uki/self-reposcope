# Self-Reposcope 🔍

![Badge](https://hitscounter.dev/api/hit?url=https%3A%2F%2Fgithub.com%2F4okimi7uki%2Fself-reposcope&label=Visitors&icon=suit-heart-fill&color=%23d63384)
![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)
[![License](https://img.shields.io/github/license/4okimi7uki/self-reposcope)](https://github.com/4okimi7uki/self-reposcope/blob/main/LICENSE)

<p align="center">
<img src="https://github.com/4okimi7uki/self-reposcope/blob/main/output/full_languages.svg" alt="stats"/>
</p>

-   GitHub の **言語使用量を SVG で可視化**する CLI ツール（Rust 製）  
    _A lightweight CLI tool built with Rust to visualize your GitHub language stats as SVG charts._
-   **プライベートリポジトリ**にも対応  
    _Supports **private repositories**_

## できること / Features

-   GitHub API を用いて、自分のリポジトリを網羅的に集計  
    _Aggregates all your repositories using the GitHub API_

-   各言語の使用量（バイト数）を積み上げ棒グラフで SVG 出力  
    _Outputs an SVG stacked bar chart based on language usage (in bytes)_

-   **プライベートリポジトリ**も対象（アクセストークン使用）  
    _Supports **private repositories** (with access token)_

-   GitHub Actions での自動集計＆更新  
    _Automatically updates via GitHub Actions_

## 背景 / Background

GitHub の使用言語統計を可視化する「Stats 系」リポジトリは多く存在します。  
しかし、その多くは **public リポジトリ限定**だったり、**導入手順が複雑**だったりと、気軽に使えるものが少ないと感じていました。

たとえば、会社アカウントと個人アカウントを使い分けている開発者にとっては、  
**private リポジトリ中心の活動が可視化されない**という課題があります。  
自分の技術スタックをもっとアピールしたいけど、それができない ——

そこで、**private リポジトリにも対応し、安全かつ「1 クリックで導入できる」ような体験を提供すること**を目指して、このツール（self-reposcope）を開発しました。

---

There are many "GitHub Stats" repositories out there that visualize language usage in your repositories.  
However, most of them are **limited to public repositories** or have **complex installation steps**,  
which makes them less accessible for casual use.

For developers who use both personal and company accounts,  
**activity in private repositories often goes unrepresented** —  
even though that’s where most of their work happens.  
You might want to showcase your real tech stack — but you simply can’t.

So I built **self-reposcope**,  
a tool that supports **private repositories** and offers a **safe, one-click setup experience**.

## 使い方 / Usage

### 🚀 GitHub Actions で自動更新（おすすめ）/ Automatic Updates via GitHub Actions (Recommended)

Repository にて、下記のように設定すると`./output`を生成し、SVG を出力します。  
_By setting up the following workflow in your GitHub repository, self-reposcope will automatically generate SVG files in the `./output` directory._

1. Repository の `Settings > Secrets and variables > Actions > [Repository secrets] > [New repository secret]` で  
   Name: `REPOSCOPE_TOKEN`, Secret: `ghp_XXXXXXXXXXXXXXX`  
   (`repo` 権限付き GitHub Token) を追加  
   _Go to `Settings > Secrets and variables > Actions > [Repository secrets]`,  
   then add a new secret with:_

    - _**Name**: `REPOSCOPE_TOKEN`_
    - _**Value**: your personal access token (with `repo` scope)_

2. [`.github/workflows/reposcope.yml`](https://github.com/4okimi7uki/self-reposcope/blob/main/.github/workflows/reposcope.yml) を作成し、以下のように記述：  
   _Create a workflow file at `.github/workflows/reposcope.yml` with the following content:_

```bash
name: Update Language Stats

on:
    schedule:
        - cron: "0 0 * * 1" # Every Monday
    workflow_dispatch:

jobs:
    build:
        runs-on: ubuntu-latest
        permissions:
            contents: write

        steps:
            - name: Checkout target repo
              uses: actions/checkout@v3

            - name: Download self-reposcope binary from GitHub Release
              shell: bash
              run: |
                  curl -L https://github.com/4okimi7uki/self-reposcope/releases/latest/download/self-reposcope -o self-reposcope
                  chmod +x ./self-reposcope

            - name: Run self-reposcope CLI
              shell: bash
              run: |
                  mkdir -p output
                  ./self-reposcope --token ${{ secrets.REPOSCOPE_TOKEN }}

            - name: Commit and Push updated SVGs
              shell: bash
              env:
                  GH_PAT: ${{ secrets.REPOSCOPE_TOKEN }}
              run: |
                  git config --global user.name 'github-actions[bot]'
                  git config --global user.email 'github-actions[bot]@users.noreply.github.com'
                  git add output/*.svg
                  if git diff --cached --quiet; then
                    echo "No changes to commit"
                  else
                    git commit -m "update: language stats svg"
                    git push https://x-access-token:${GH_PAT}@github.com/${{ github.repository }} HEAD:main
                  fi
```

3. 手動実行(`Actions` > `Update Language Stats` > `Run workflow`) または 自動で毎週更新されます。差分がなければ新規出力されません。

    _You can run the workflow manually (`Actions > Update Language Stats > Run workflow`), or it will automatically run every Monday.
    If there are no changes in the output, nothing will be committed._

### 🧪 ローカルで試す / Try It Locally (Rust CLI)

> 実行環境 / Requirements
>
> -   Rust 1.87.0+ (with `cargo`)

1. リポジトリに対してアクセス権のある GitHub token を用意  
   _**Prepare a GitHub personal access token** with access to your repositories_
2. このリポジトリをクローン  
   _**Clone this repo**_
3. `.env`ファイルを作成して、GitHub token を設定:  
   _**Add your GitHub token** to `.env`_

```env
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxx
```

4. 下記コマンドで実行:  
   _**Run with:**_

```bash
cargo run --release
```

5. `./output`に`*.svg`ファイルが出力されていることを確認  
   _Check that the `*.svg` files are generated in the `./output` directory_

---

<small>2025 [Aoki Mizuki](https://github.com/4okimi7uki) – Developed with 🍭 and a sense of fun.</small>
