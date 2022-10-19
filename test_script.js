#!/usr/bin/env node

const n = Math.random();

if (n < 0.5) {
  throw new Error(n);
}
