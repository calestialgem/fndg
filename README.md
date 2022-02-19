# Founding

An **unchained** map game where you battle for resources while _building_ your
_nation_ up.

## Features

Here "unchained" means that there will be no arbitrary limits on the quantity of
stuff like units and cities. This is the game's main aim, which spills to
_modding support_ as well. Although, the scope is very small for now.

The prototype will have the minimum amount of features: _map generation_ from
perlin noise, building and capturing _cities_, recruiting _armies_, and a simple
_enemy AI_ to play againts.

If all these can be implemented, more economy focused features will be done:
_roads_, _buildings_, _city improvements_, _sailing_. A third set of features,
which are about incresing the depth of the game, can be done as time allows.
These will be _technology_, _diplomacy_, and _multiplayer_.

## Game Engine

> Using **Bevy** as the powering technology.

Not anymore; since 0.5.0. **Bevy** is a _great_ game engine, I loved its design,
but I do not have time to learn how to render using it, unfortunetely. The frame
rate was very bad because of that. Furthermore, I could not even render the
lines in the map properly.

Rather than spending more time on learning a third new thing (**Bevy** uses
**WebGL** I suppose), I decided to render in **OpenGL**, and the only new thing
would be **Rust**. While I just started again, writing min_gl and min_timer
helped a lot.

Writing code to work with **Bevy** was a different experience that writing my
own game framework. The reason is, I get to make most of the design desicions
and this pushed me to read and learn about **Rust**, rather than just making it
compile and work properly. (I could not get there anyways even with something as
basic as rendering a line mesh.)

## License

Licensed under either of

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Author

Keep in mind that I'm very new to **Rust** (and **Bevy**, as you can imagine; I
don't use it anymore tho)! I programmed in **Java** for 7-8 years, then tried
some **C/C++**, and ended up in **Rust**.

> I create games and programs for _hobby_, I cannot go at it full-time. I will
> try to get atleast a prototype running in _2 weeks_. After that the
> development _might stall_ because of the university.

That aged well! The "2 weeks" are over, and I'm starting clean! Altough, I could
not get something playable in the end, I enjoyed trying and learning.

Copyright 2022 Cem Geçgel <gecgelcem@outlook.com>
