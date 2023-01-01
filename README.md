# PestCömp

https://user-images.githubusercontent.com/26331682/210181264-700439aa-5661-44e5-a357-a8c4b7d2bec1.mp4

[^song]

Forget your Pro-C2, Presswerk or Shadow Hills Mastering Compressor. If your song sounds like shit, those compressors could never make it sound less shit. PestCömp does just that by carefully analyzing the input signal's frequencies and turning them into their closest certified good-sounding counterparts. Not only that, it applies slight dynamics processing through extremely accurate analog modelling of an unplugged tape deck.[^1]

In short, it turns your terrible music into complete silence, therefore making it slightly more listenable and saving you from the inevitable embarassment you'd get from playing it to an audio engineer. In fact, the entirety of PestCömp's DSP can be summed up through a single line of code:

```cpp
for (uint32_t i = 0; i < frames; ++i) {
    outL[i] = 0;
    outR[i] = 0;
}
```

This plugin was entirely rewritten for version 2.0. You can check the original Rust code in the [old branch](https://github.com/bandithedoge/pestcomp/tree/master).

[^1]: Okay, excuse my terrible comedy. I obviously wrote this plugin as a stupid inside joke that isn't even worth explaining.

[^song]: Song used in demo: [*Repulsion - Slaughter of the Innocent*](https://www.youtube.com/watch?v=mm2SDuKjGVA)
