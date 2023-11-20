# MANDELBROT

## Single-thread

```zsh
time target/release/mandlebrot mandelbrot-00.png 4000x3000 -1.20,0.35 -1.0,0.20  3.56s user 0.01s system 91% cpu 3.901 total

time target/release/mandlebrot mandelbrot-02.png 4000x3000 -1.11,0.32 -1.0,0.20  4.77s user 0.01s system 99% cpu 4.806 total

time target/release/mandlebrot mandelbrot-03.png 4000x3000 -1.20,0.35 -1.5,0.20  0.20s user 0.01s system 92% cpu 0.224 total
```

## Concurrent

```zsh
time target/release/mandlebrot mandelbrot-concurrent-01.png 4000x3000 -1.20,0.35  3.61s user 0.01s system 90% cpu 3.983 total
```
