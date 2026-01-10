<!-- SPDX-License-Identifier: MIT -->
<!-- SPDX-FileCopyrightText: Copyright 2026 Sam Blenny -->
# Dabao Zephyr CircuitPython PoC

**DRAFT: WORK IN PROGRESS**

[*an aspirational blurb about what this is trying to be*:]

This is a proof of concept for building CircuitPython's Zephyr port to run on
the Baochip Dabao board. Most of the work here is about getting Rust no_std
device drivers from the Dabao bootloader to work as Zephyr drivers. There's
some additional stuff for getting the CircuitPython Zephyr port to build with
my Zephyr board definition for Dabao.

**CAUTION:**

1. This is a proof of concept. I don't currently have any intention to maintain
   this once I've got it working. The point is to document how to do it.

2. The Zephyr Project has strict contribution policies which I am not
   following. Commits in this repo are unsuitable for upstreaming. But...

3. I'm arranging things here to hopefully facilitate easy re-implementation or
   translation of my work (e.g. Rust to C with a new copyright). If you want to
   do that and follow the Zephyr Project upstreaming policies, go for it. Best
   of luck.
