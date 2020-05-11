#!/usr/bin/env bash
cd pkg
npm version patch
npm publish
cd ..