## Game example made in rust and amethyst

This is a small game made as my course work for Kyiv Mohyla Academy.

It can be used as an example of a very simple game, but it is for sure not the best example of how to use ECS, so be careful.  

### How to run
First you will need to install [rust](https://www.rust-lang.org/).

Then in cloned repository run command:
```
cargo run
```

### Current state
Things that are ready:
* Animation components, systems
* Basic hero components, system (input system, moving, hitting)
* Regular enemy component, systems (actions decision, finding direction to hero, moving, hitting)
* One example of regular enemy
* UI: start menu

Plan for near time:
* Hit register for both enemy and hero
* UI: health bars
* Skill system to make game more advanced example
* Add more enemies examples

#### Sources that was used or can be useful 

* [Rust book](https://doc.rust-lang.org/book/)
* Amethyst [repository](https://github.com/amethyst/amethyst) | [book](https://www.amethyst.rs/book/master/) | [discord server](https://discordapp.com/invite/WzHFX3)
* [Great 2d artist sprites of whom used in game](https://opengameart.org/users/calciumtrice) 