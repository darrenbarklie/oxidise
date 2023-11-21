# MANDELBROT

## Single-thread

```zsh
time target/release/mandlebrot mandelbrot-01.png 4000x3000 -1.20,0.35 -1.0,0.20  3.56s user 0.01s system 91% cpu 3.901 total

time target/release/mandlebrot mandelbrot-02.png 4000x3000 -1.11,0.32 -1.0,0.20  4.77s user 0.01s system 99% cpu 4.806 total

time target/release/mandlebrot mandelbrot-03.png 4000x3000 -1.20,0.35 -1.5,0.20  0.20s user 0.01s system 92% cpu 0.224 total
```

## Concurrent (bands)

```zsh
time target/release/mandlebrot mandelbrot-11.png 4000x3000 -1.20,0.35 -1.0,0.20  4.18s user 0.03s system 319% cpu 1.317 total

time target/release/mandlebrot mandelbrot-12.png 4000x3000 -1.11,0.32 -1.0,0.20  5.42s user 0.02s system 360% cpu 1.510 total

time target/release/mandlebrot mandelbrot-13.png 4000x3000 -1.20,0.35 -1.5,0.20  0.28s user 0.01s system 295% cpu 0.100 total
```
