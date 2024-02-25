mod player;

use godot::prelude::*;

struct HelloRusty;

#[gdextension]
unsafe impl ExtensionLibrary for HelloRusty {}
