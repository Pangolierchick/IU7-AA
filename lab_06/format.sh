#!/bin/sh

cd "${MESON_SOURCE_ROOT}"

clang-format -i --style=file ./src/*
