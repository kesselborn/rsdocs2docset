[![Build Status](https://travis-ci.org/kesselborn/rsdocs2docset.svg?branch=master)](https://travis-ci.org/kesselborn/rsdocs2docset) [![Build status](https://ci.appveyor.com/api/projects/status/a33lro8pqv3p2t2x?svg=true)](https://ci.appveyor.com/project/kesselborn/rsdocs2docset)

# rsdocs2docset

This is a small tool to generate a docsets for [Dash](https://kapeli.com/dash) / [Velocity](http://velocity.silverlakesoftware.com) / [Zeal](https://zealdocs.org) / [LovelyDocs](https://zealdocs.org) out of Rust docs.

# installation
Simply call

    cargo install rsdocs2docset

or clone this repo, make sure you have rust installed and execute

    cargo install

# usage

given your docs live in `target/doc`, execute

    rsdocs2docset --rsdocs target/doc --name mycrate

and `rsdocs2docset` will create a folder called `mycrate.docset` ... or, given you have rustup and a nighlty rust version installed,
use the included [./create-rs-docset](create-rs-docset) script to create docs for a specific package that can be found
using cargo:

    ./create-rs-docset html5ever

This can take long the first time, but it caches build artifacts in `~/.rs-docsets`, so it should get faster over time.
This is a stepping stone for now

