# PestCömp

https://user-images.githubusercontent.com/26331682/180800956-4a0db6c4-36d7-48fb-938d-d39dac427a2b.mp4

[^song]

Forget your Pro-C2, Presswerk or Shadow Hills Mastering Compressor. If your song sounds like shit, those compressors could never make it sound less shit. PestCömp does just that by carefully analyzing the input signal's frequencies and turning them into their closest certified good-sounding counterparts. Not only that, it applies slight dynamics processing through extremely accurate analog modelling of an unplugged tape deck.

In short, it turns your terrible music into complete silence, therefore making it slightly more listenable and saving you from the inevitable embarassment you'd get from playing it to an audio engineer. In fact, the entirety of PestCömp's DSP can be summed up through a single line of code:

```rust
// as far as I know, VST2 only supports float parameters so this is the only
// way to make something that resembles a boolean
*output_sample = if engage < 0.5 { *input_sample } else { 0.0 };
```

^Okay, excuse my terrible comedy. I obviously wrote this plugin as a stupid inside joke that isn't even worth explaining.^

[^song]: Song used in demo: [*Repulsion - Slaughter of the Innocent*](https://www.youtube.com/watch?v=mm2SDuKjGVA)
