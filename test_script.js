#!/usr/bin/env node

const n = Math.random();

if (n < 0.2) {
  throw new Error(n);
}
