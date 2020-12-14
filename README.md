# Running

- Make sure you have Rust installed via [rust-lang.org](https://www.rust-lang.org/learn/get-started).

- Install the `cargo-aoc` runner via `cargo install cargo-aoc`.

  - This allows you to run `cargo aoc`, which will automatically download and cache the input

- Run the latest implemented day with `cargo aoc`, or run a specific day with `cargo aoc -d <day>`. Automatic benchmarking can be done with `cargo aoc bench -d <day>`, or `cargo aoc bench` for the latest implemented day.

# Observations

## Day 2 - Password policies

This problem was relatively easy by Advent of Code standards. However, the second part is a great vehicle to make a couple of quick remarks about mental models of programming, string indexing, and how optimizations work in rust.

### The problem

We're given a string, a character, and two numbers.

```rust
struct Policy {
    min: usize,
    max: usize,
    character: char,
    password: String,
}
```

We're told that the `character` must either be in the `min` position or the `max` position in the `password` string - not both. The first implementation ran in around 80 microseconds, while the later optimized version runs in around 25 microseconds.

### Initial implementation

My initial plan was based on the fact that memory lookups are fast and almost free when they're in the cache - something the short passwords used in this problem would be. So my strategy became the following:

1. Get the characters of the string
2. Check that the character is at index 1, but not index 2, or check that the character is at index 2 but not index 1.

Easy to understand, easy to implement. In fact, even a programmer that doesn't know Rust could probably read and understand the check.

```rust
(chars[self.min - 1] == self.character && chars[self.max - 1] != self.character)
    || (chars[self.min - 1] != self.character && chars[self.max - 1] == self.character)
```

Because of the semantics of the Rust language the compiler knows that `chars[i]` will always result in the same value, and I know that this fact is something both the compiler and LLVM uses when optimizing. Which means that the whole function is four comparisons and just 2 memory lookups. That's without taking short-circuiting into consideration.

So why is this not the fastest option?

### Iteration

A part of the above implementation that I didn't share is where `chars` comes from. It turns out that the whole "get the characters of the string" is rather critical to the performance of the piece of code. See, a `String` in Rust cannot be indexed directly. The reason being is that every `String` is guaranteed to be valid UTF-8, and indexing into such a structure is not entirely well-defined. Byte-indexing would quickly break in user-code, and since UTF-8 is a variable-length encoding an index operation on characters would naïvely have to scan the string.

To solve this Rust strings has the `.chars()` method, which creates an _iterator_ over the characters of the string. Iterators in Rust behave much in the same way as `Iterator<T>` in Java. To get an indexable collection of the characters I essentially needed to iterate through every character, and push it to a `Vec` - a `List<T>` by another name. It looks like this:

```rust
let chars: Vec<char> = self.password.chars().collect();
```

`.collect()` does the pushing to the `Vec` automatically. The answer is allocation. `.collect()` goes and asks the operating system to allocate some amount of memory on the heap. And if we're really unlucky it might even have to move and resize the backing buffer as the characters are pushed into it. Additionally, we can't get rid of the `self.password.chars()` call because we need to take a look at individual characters arbitrarily placed in the string.

### Avoiding allocations

My solution to this debacle was a change of mindset. I don't need to "get the characters of the string". I need to evaluate each character, and it's associated index in the string, independently of anything else. Importantly, the `self.password.chars()` iterator doesn't allocate on the heap since it manually looks through the bytes and extracts one character at the time - forgetting them as it goes along. Since we also need to know the index of each character we can do this by having a running count like this:

```rust
let mut chrs = self.password.chars();
let mut idx = 0;
while let Some(chr) = chrs.next() {
  ...
  idx += 1;
}
```

However, this is a common enough occurrence that a `.char_indices()` method exists on strings, that gives us an iterator of tuples with an index and the character. In other words, we're only interested in the characters at the two indices we're given, and then we want to know how many of those - 0, 1, or 2 - are the specified character. Only if exactly one of them are the character is the password valid. What this looks like in Rust:

```rust
self.password
    .char_indices()
    .filter(|(idx, _)| *idx == self.min || *idx == self.max)
    .filter(|(_, ch)| *ch == self.character)
    .count()
    == 1
```

The syntax `|param| expression` is how a closure is written. In Java you'd write `(param) -> expression`. The parenthesis are unpacking the tuple from the iterator, as we only care about one fo the tuple elements in each `.filter` call. Do note that the two calls to `.filter` can be combined, but quick examination of the generated assembly shows that the two generate exactly the same code. The code show here, however, is more readable.

### Why is this faster?

At first glance you might think we're doing more work here. For every character we're doing at least one comparison. With a quick glance at my puzzle input I can tell you that the shortest given input is 4 characters long. Let's quickly figure out how many comparisons we do on a string of length `N`.

We have `A || B` when we aren't looking at any of our two indices, thus both of the comparisons need to be made. That's at least `(N - 2) * 2` comparisons. When we are looking at one of the two indices one of them will be the first check in `A || B`, and will thus short-circuit. The other index will not short circuit, so that's another two. Additionally, both of the indicies will also do the comparison on the character itself. That's 5 comparisons for the two indicies. That's `(N - 2) * 2 + 5` comparisons in total, compared to the just 4 of the initial implementation.

So, we're faster than our initial implementation by virtue of not doing any the allocation of the vector. But maybe there's a way in which we can use the time saved on comparisons to move our characters into easily indexable memory?

### Modifications to the initial implementation

The idea is simple. Instead of letting the vector grow as we push into it, we allocate the full capacity up-front. After all, we do know how long the string is. We change the initialization fo the `chars` vector to this:

```rust
let mut chars = Vec::with_capacity(self.password.len());
chars.extend(self.password.chars());
```

The capacity is already there, so the only thing the program has to do is move the characters onto the heap as they're extracted by the iterator.

Alas, while this is _significantly_ faster than the original implementation, it reaches 31 microseconds, it's not as fast as the previously found iterator-based code which reaches 25 microseconds. Why?

### Security? Gah

I lied earlier. Our original implementation doesn't do 4 comparisons and 2 memory lookups. It does 6 comparisons and 2 memory lookups. Rust doesn't allow out-of-bounds indexing, and the program crashes when this is attempted. In order to crash, the program must check that the index asked for is within bounds. Ordinarily, the optimizer is incredibly good at removing these checks if it can tell where the index comes from. But in our code there is no explicit connection, to the compiler, between the indicies we're using and the length of the vector. Thus it needs to check the bounds for every lookup. This, along with the relatively short input passwords and the overhead of moving characters onto the heap, simply doesn't allow indexing to truly shine.

### Who needs security anyway?

Well, in general we're probably pretty happy with bound checks in our code - Heartbleed anyone? But in this case I do trust both my input parsing and the input that has been provided to me by Advent of Code. So why not disable the checks for just these 2 lookups?

```rust
(*unsafe { chars.get_unchecked(self.min - 1) } == self.character
    && *unsafe { chars.get_unchecked(self.max - 1) } != self.character)
    || (*unsafe { chars.get_unchecked(self.min - 1) } != self.character
        && *unsafe { chars.get_unchecked(self.max - 1) } == self.character)
```

### How's this then?

It's better. By 1.5 microseconds. Still consistently slower than the filter-based approach. I've reached a point where I assume any further improvements would come from rather arcane optimizations, or from simply using trying longer inputs.

### How useful is this?

It's not. At all. Microbenchmarks are never truly indicative of anything, and any optimization in a real program should be guided by profiling data - not "I think"s. That said, I find it quite fascinating to see how seemingly inconsequential coding decisions impact performance at the lowest level.
