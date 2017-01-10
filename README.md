[![Build Status](https://travis-ci.org/kesselborn/rsdocs2docset.svg?branch=master)](https://travis-ci.org/kesselborn/rsdocs2docset)
# rsdocs2docset

This is a small tool to generate a docsets for [Dash](https://kapeli.com/dash) / [Velocity](http://velocity.silverlakesoftware.com) / [Zeal](https://zealdocs.org) / [LovelyDocs](https://zealdocs.org) out of Rust docs.

# installation

Clone this repo, make sure you have rust installed and execute

    cargo install

# usage

given your docs live in `target/doc`, execute

    rsdocs2docset --rsdocs target/doc --name mycrate

and `rsdocs2docset` will create a folder called `mycrate.docset`
