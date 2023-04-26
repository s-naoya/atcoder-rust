#!/bin/bash
# 事前準備
# source command.sh

export CONTEST=$(basename $PWD)

# コンテストに応じたパッケージの生成
# new abc100
function new() {
    command cargo compete new $1
    command cp -r .vscode.temp $1/.vscode
    command code $1
}
# テスト実行
# test a
function test() {
    command cargo compete test $1
}
# 提出
# submit a
function submit() {
    command cargo compete submit $1
}
# 提出（テスト実行、提出状況表示なし）
# submit-nn a
function submit-nn() {
    command cargo compete submit --no-test --no-watch $1
}
function run() {
    command cargo run --bin ${CONTEST}-$1
}