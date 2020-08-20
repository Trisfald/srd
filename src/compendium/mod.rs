//! Compendium module.

pub mod standard_compendium;
pub use self::standard_compendium::StandardCompendium;

use crate::character::{ClassId, ClassModel, RaceId, RaceModel};
use crate::{AbilityId, SkillId};
use crate::{SRDError, SRDResult};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Contains all extensible rules definitions.
pub trait Compendium {
    /// Returns the version of this compendium.\
    /// This will be used to compute the version of the battle rules.
    fn version(&self) -> u32 {
        0
    }

    /// Returns all existing abilities.
    fn abilities<'a>(&'a self) -> Box<dyn Iterator<Item = &AbilityId> + 'a> {
        Box::new(std::iter::empty())
    }

    /// Returns all existing skills.
    fn skills<'a>(&'a self) -> Box<dyn Iterator<Item = &SkillId> + 'a> {
        Box::new(std::iter::empty())
    }

    /// Returns the ability associated to the given skill, or `None` if the skill doesn't exist.
    fn associated_ability(&self, _: &SkillId) -> Option<&AbilityId> {
        None
    }

    /// Returns all existing character races.
    fn races<'a>(&'a self) -> Box<dyn Iterator<Item = &RaceId> + 'a> {
        Box::new(std::iter::empty())
    }

    /// Returns the model describing the given race, or `None` if the race doesn't exist.
    fn race_model(&self, _: &RaceId) -> Option<&dyn RaceModel> {
        None
    }

    /// Returns all existing character classes.
    fn classes<'a>(&'a self) -> Box<dyn Iterator<Item = &ClassId> + 'a> {
        Box::new(std::iter::empty())
    }

    /// Returns the model describing the given class, or `None` if the class doesn't exist.
    fn class_model(&self, _: &ClassId) -> Option<&dyn ClassModel> {
        None
    }
}

struct EmptyCompendium;

impl Compendium for EmptyCompendium {}

// Code for initializing a compendium as a global variable.
// We took the log crate as inspiration.

// The COMPENDIUM static holds a pointer to the global compendium. It is protected by
// the COMPENDIUM static which determines whether COMPENDIUM has been initialized yet.
static mut COMPENDIUM: &dyn Compendium = &EmptyCompendium;

static STATE: AtomicUsize = AtomicUsize::new(0);

const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;

/// Sets the global compendium to a `Box<Compendium>`.
///
/// This is a simple convenience wrapper over `set_compendium`, which takes a
/// `Box<Compendium>` rather than a `&'static Compendium`. See the documentation for
/// [set_compendium](fn.set_compendium.html) for more details.
///
/// # Errors
///
/// An error is returned if a compendium has already been set.
pub fn set_boxed_compendium(compendium: Box<dyn Compendium>) -> SRDResult<()> {
    set_compendium_inner(|| unsafe { &*Box::into_raw(compendium) })
}

/// Sets the global compendium to a `&'static Compendium`.
///
/// This function may only be called once in the lifetime of a program. It is
/// advised to call this function after a logging system has been initialized.
///
/// If now compendium has been set, a empty one will be used.
///
/// # Errors
///
/// An error is returned if a compendium has already been set.
///
/// # Examples
///
/// ```
/// use srd::{AbilityId, Compendium, compendium, set_compendium};
///
/// static MY_COMPENDIUM: MyCompendium = MyCompendium;
///
/// struct MyCompendium;
///
/// impl Compendium for MyCompendium {
///     fn abilities<'a>(&'a self) -> Box<dyn Iterator<Item = &AbilityId> + 'a> {
///         Box::new(std::iter::once(&AbilityId(1)))
///     }
/// }
///
/// fn main() {
///     set_compendium(&MY_COMPENDIUM).unwrap();
///     assert_eq!(compendium().abilities().count(), 1);
/// }
/// ```
#[cfg(atomic_cas)]
pub fn set_compendium(compendium: &'static dyn Compendium) -> SRDResult<()> {
    set_compendium_inner(|| compendium)
}

#[cfg(atomic_cas)]
fn set_compendium_inner<F>(make_compendium: F) -> SRDResult<()>
where
    F: FnOnce() -> &'static dyn Compendium,
{
    match STATE.compare_and_swap(UNINITIALIZED, INITIALIZING, Ordering::SeqCst) {
        UNINITIALIZED => {
            unsafe {
                COMPENDIUM = make_compendium();
            }
            STATE.store(INITIALIZED, Ordering::SeqCst);
            log::info!("registered a compendium");
            Ok(())
        }
        INITIALIZING => {
            while STATE.load(Ordering::SeqCst) == INITIALIZING {
                std::sync::atomic::spin_loop_hint();
            }
            Err(SRDError::SetCompendiumError)
        }
        _ => Err(SRDError::SetCompendiumError),
    }
}

/// Convenience method to initialize and set a `StandardCompendium` containing all the
/// rules from the SRD.
///
/// # Panics
///
/// The method panics if another compendium has been already set.
///
/// # Examples
///
/// ```
/// use srd::{compendium, init_srd_compendium};
///
/// fn main() {
///     init_srd_compendium();
///     assert_eq!(compendium().abilities().count(), srd::ability::RESERVED_ABILITIES.into());
/// }
/// ```
pub fn init_srd_compendium() {
    set_boxed_compendium(Box::new(StandardCompendium::with_srd()))
        .expect("failed to set the compendium!");
}

/// Returns a reference to the compendium.
///
/// If a compendium has not been set, a no-op implementation is returned.
pub fn compendium() -> &'static dyn Compendium {
    if STATE.load(Ordering::SeqCst) != INITIALIZED {
        static NOP: EmptyCompendium = EmptyCompendium;
        &NOP
    } else {
        unsafe { COMPENDIUM }
    }
}
