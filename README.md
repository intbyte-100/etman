# Etman (Easy Text MANipulator)

Etman is meant to be an easier `awk`/`sed` but the real point of it isn't the syntax, it's the execution model underneath. It's a streaming engine for processing large (or effectively unbounded) text data, and the language sits on top of that as a way to write pipelines, parse formats, and query data.

The project is very early. There's barely any code yet, most of what exists right now is design notes on the language and the execution model, and both are still likely to change.

## Why not just use awk/sed

`awk` and `sed` run your script line by line, in order. Etman is built around a different idea: a pipeline is a *description* of what you want, not a sequence of steps to run directly. An engine decides how to actually execute it, splitting the input into chunks (~64 KB), scheduling chunk processing across workers (bag-of-tasks style), and reassembling the output in the same order you'd get from running it sequentially. So a one-liner behaves the way you'd expect, but the same model also scales to data that doesn't fit in memory.

## What the code looks like

Pull lines starting with `"User:"`, strip the prefix, split into words:

```rust
file "file.txt"
	| lines
	| where starts_with "User:"
	| erase "User:"
	| words
	| write "new.txt" "$it;\n"
```

A query over JSON:

```rust
let cenzure = (word: text) => bool {
	text | words | any | in cached (file "black_list.txt" | words | set)
}

let querry = $[
	./messages/.../

	where ./id in ["intbyte", "016"]
		and ./type == "message"
		and cenzure(./text)

	map {
		./id = ./id
		./message = ./text
	}

	../
]

file "tg-messages.json" | json | querry | to-json | write "new.json"
```

A named stream:

```shell
stream to-int-and-square: text -> number {
	input | to-int | crash-on-error | expr 'it^2' | out
}
```

## How execution works (design, not implementation yet)

A pipeline is built out of three kinds of units:

- **Generators** (1 → N)  take one element and can produce several, e.g. `lines`, `words`.
- **Processors** (1 → 1)  transform an element without changing the count.
- **Predicates** (1 → 0 or 1) filter the stream, keeping or dropping each element.
- A **terminal operation** at the end materializes the result, in the correct order.

A few things the engine is responsible for:

**Element identity.** Every element carries a `(source_id, generation_id)` pair where it came from in the original stream. `source_id` tracks the element's origin, not its current position, so ordering is a property of the data itself rather than of whatever order tasks happened to finish in. `generation_id` only comes into play when a single input produces multiple outputs (e.g. `words` splitting a line).

**Chunking.** The engine batches whatever a generator produces into chunks of roughly 64 KB, and that's the unit both scheduling and completion-tracking happen at not individual elements, not pipeline stages. If a generator can't fit what it's producing into a chunk (a single line longer than that, say), it hands the stream itself further down the pipeline instead, and that section runs synchronously until the boundary is found and chunking can resume.

**Independence between units.** Once a unit gets its data, it runs without knowing anything about the state of the units around it, no cross-unit synchronization. The engine handles routing, scheduling, and keeping the overall pipeline semantics correct.

**Finite vs. streaming data.** Some data has a known size and can just sit in memory (finite); some has to be processed as it arrives and might be unbounded (streaming). A `line` is usually finite, but if a file is one very long line, it gets treated as a stream instead. Anything that can handle streaming data can also handle finite data.

## Status

Nothing here is stable this is roughly the order things need to get built in:

- data model (finite vs. streaming)
- pipeline execution engine + chunking
- language parser
- core generators/processors/predicates (`lines`, `words`, `where`, `erase`, `regex`, ...)
- format support (JSON, CSV, XML, YAML)
- the query syntax (`$[ ... ]`)
- named streams (`stream ... { ... }`)

