use crate::prelude::*;
use rand::distributions::WeightedIndex;

pub struct SpawnTable {
    templates: Vec<CharacterTemplate>,
    weighted_index: WeightedIndex<i32>,
}

impl SpawnTable {
    pub fn new(spawn_table: &[(i32, &CharacterTemplate)]) -> Self {
        let (weights, templates): (Vec<_>, Vec<_>) = spawn_table.iter().copied().unzip();

        Self {
            templates: templates.into_iter().cloned().collect(),
            weighted_index: WeightedIndex::new(weights).expect("invalid weights for spawn table"),
        }
    }
}

impl Distribution<CharacterTemplate> for SpawnTable {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CharacterTemplate {
        self.templates[self.weighted_index.sample(rng)].clone()
    }
}
