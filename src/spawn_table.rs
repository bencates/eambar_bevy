use {
    crate::{assets::TextSprite, character::CharacterBundle},
    rand::{
        distributions::WeightedIndex,
        prelude::{Distribution, Rng},
    },
};

pub type SpawnFn = fn(&TextSprite) -> CharacterBundle;

pub struct SpawnTable {
    spawns: Vec<SpawnFn>,
    weighted_index: WeightedIndex<i32>,
}

impl SpawnTable {
    pub fn new(spawn_table: &[(i32, SpawnFn)]) -> Self {
        let (weights, spawns): (Vec<_>, Vec<_>) = spawn_table.iter().copied().unzip();

        Self {
            spawns,
            weighted_index: WeightedIndex::new(weights).expect("invalid weights for spawn table"),
        }
    }
}

impl Distribution<SpawnFn> for SpawnTable {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnFn {
        self.spawns[self.weighted_index.sample(rng)]
    }
}
