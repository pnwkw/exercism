// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Self {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana > mana_cost {
                self.mana = Some(mana - mana_cost);

                2 * mana_cost
            } else {
                0
            }
        } else {
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }

            0
        }
    }
}
