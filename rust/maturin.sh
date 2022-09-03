#!/usr/bin/env bash
mkdir nettle_py
cd "$_"
python3 -m venv .env
source .env/bin/activate
pip install maturin
maturin init --bindings pyo3
maturin develop
