#!/usr/bin/env bash

RUSTDOCFLAGS="--document-private-items" cargo +stable doc "$@"
