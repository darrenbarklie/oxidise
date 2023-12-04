# INTRO

When it comes to managing memory, there are two desirable characteristics:
- Memory is freed promptly
- Never want to use a pointer to an object after it's been freed

However traditionally these are mutually exclusive: freeing a value while
pointers exist to it necessarily leaves those points dangling.

- The "safety first" camp uses garbage collection to manage memory,
  automatically freeing objects when all reachable pointers to them are gone.
  This eliminates dangling points by simply keeping the objects around until
  there are no pointers to them left to dangle. Almost all modern languages
  fall in this camp: Python, JavaScript, Ruby, Java, C#, Haskell.

- The "Control First" camp leaves you in charge of freeing memory. The
  program's memory consumption is entirely in your hands, but avoiding
  dangling pointers also becomes entirely the developers concern. C and C++
  are the only mainstream languages in this camp.
