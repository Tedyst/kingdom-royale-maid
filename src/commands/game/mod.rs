mod join;
mod new_game;

pub use join::*;
pub use new_game::*;

mod prelude {
    pub use crate::commands::prelude::*;
    pub use crate::game::Game;

    pub struct GameContainer;

    impl TypeMapKey for GameContainer {
        type Value = Arc<RwLock<Game>>;
    }
}
