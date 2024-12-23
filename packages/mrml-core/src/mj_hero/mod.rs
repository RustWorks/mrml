use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-hero";

pub struct MjHeroTag;

impl StaticTag for MjHeroTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjHero = Component<PhantomData<MjHeroTag>, crate::prelude::AttributeMap, Vec<MjBodyChild>>;
