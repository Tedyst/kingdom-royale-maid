use super::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoleName {
    King,
    Prince,
    TheDouble,
    Sorcerer,
    Knight,
    Revolutionary,
}

impl RoleName {
    pub fn is_king_like(&self) -> bool {
        match self {
            RoleName::King | RoleName::TheDouble | RoleName::Prince => true,
            _ => false,
        }
    }
}

impl ToString for RoleName {
    fn to_string(&self) -> String {
        match self {
            RoleName::King => "King".to_string(),
            RoleName::Prince => "Prince".to_string(),
            RoleName::TheDouble => "The Double".to_string(),
            RoleName::Sorcerer => "Sorcerer".to_string(),
            RoleName::Knight => "Knight".to_string(),
            RoleName::Revolutionary => "Revolutionary".to_string(),
        }
    }
}

pub trait Role {
    fn name(&self) -> RoleName;
    fn win_condition_achieved(&self, game: &Game) -> bool;
}

impl Into<DeathCause> for RoleName {
    fn into(self) -> DeathCause {
        match self {
            RoleName::Sorcerer => DeathCause::Sorcery,
            RoleName::Knight => DeathCause::Beheading,
            RoleName::Revolutionary => DeathCause::Assassination,
            _ => panic!(
                "RoleName.into::<KilledBy> should never be called on {:?}",
                self
            ),
        }
    }
}
