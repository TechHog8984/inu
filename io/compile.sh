#!/bin/bash

if [[ $(pwd) = *io ]]; then
	prefix="."
else
	prefix="io"
fi

# luac5.1.5 -s -o "$prefix/input.luac" "$prefix/input.lua"
luac5.1.5 -o "$prefix/input.luac" "$prefix/input.lua"
