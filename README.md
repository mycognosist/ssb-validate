[![Build Status](https://travis-ci.org/sunrise-choir/ssb-validate.svg?branch=master)](https://travis-ci.org/sunrise-choir/ssb-validate)
# ssb-validate

> Verify Secure Scuttlebutt (SSB) hash chains (in parallel)

## Docs

[Rustdocs](https://sunrise-choir.github.io/ssb-validate/ssb_validate/index.html)

## About

Secure Scuttlebutt "feeds" are a sequence of messages published by one author.
To be a valid message,
- each message must include the hash of the preceding message
- the sequence number must be one larger than sequence of the preceding message
- the author must not change compared to the preceding message
- If it's the first message in a feed, the sequence must be 1 and the previous must be null.
- If the message includes the key, it must be that hash of the value of the message..

You can check messages one by one or batch process a collection of them (uses [rayon](https://docs.rs/rayon/1.2.0/rayon/index.html) internally)
