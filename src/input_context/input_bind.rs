use std::iter;

use super::{input_condition::InputCondition, input_modifier::InputModifier};
use crate::input::Input;

/// Associated input for [`ActionBind`](super::context_instance::ActionBind).
#[derive(Debug)]
pub struct InputBind {
    pub input: Input,
    pub modifiers: Vec<Box<dyn InputModifier>>,
    pub conditions: Vec<Box<dyn InputCondition>>,

    /// Newly created mappings are ignored by default until until a zero
    /// value is read for them.
    ///
    /// This prevents newly created contexts from reacting to currently
    /// held inputs until they are released.
    pub(super) ignored: bool,
}

impl InputBind {
    /// Creates a new instance without modifiers and conditions.
    pub fn new(input: impl Into<Input>) -> Self {
        Self {
            input: input.into(),
            modifiers: Default::default(),
            conditions: Default::default(),
            ignored: true,
        }
    }
}

impl<I: Into<Input>> From<I> for InputBind {
    fn from(input: I) -> Self {
        Self::new(input)
    }
}

/// A trait to ergonomically add a modifier or condition to any type that can be converted into a binding.
pub trait InputBindModCond {
    /// Adds modifier.
    #[must_use]
    fn with_modifier(self, modifier: impl InputModifier) -> InputBind;

    /// Adds condition.
    #[must_use]
    fn with_condition(self, condition: impl InputCondition) -> InputBind;
}

impl<T: Into<InputBind>> InputBindModCond for T {
    fn with_modifier(self, modifier: impl InputModifier) -> InputBind {
        let mut binding = self.into();
        binding.modifiers.push(Box::new(modifier));
        binding
    }

    fn with_condition(self, condition: impl InputCondition) -> InputBind {
        let mut binding = self.into();
        binding.conditions.push(Box::new(condition));
        binding
    }
}

/// Represents collection of bindings that could be passed into
/// [`ActionBind::to`](super::context_instance::ActionBind::to).
///
/// Can be manually implemented to provide custom modifiers or conditions.
/// See [`preset`](super::preset) for examples.
pub trait InputBindings {
    /// Returns an iterator over bindings.
    fn iter_bindings(self) -> impl Iterator<Item = InputBind>;
}

impl<I: Into<InputBind>> InputBindings for I {
    fn iter_bindings(self) -> impl Iterator<Item = InputBind> {
        iter::once(self.into())
    }
}

impl<I: Into<InputBind> + Copy> InputBindings for &Vec<I> {
    fn iter_bindings(self) -> impl Iterator<Item = InputBind> {
        self.as_slice().iter_bindings()
    }
}

impl<I: Into<InputBind> + Copy, const N: usize> InputBindings for &[I; N] {
    fn iter_bindings(self) -> impl Iterator<Item = InputBind> {
        self.as_slice().iter_bindings()
    }
}

impl<I: Into<InputBind> + Copy> InputBindings for &[I] {
    fn iter_bindings(self) -> impl Iterator<Item = InputBind> {
        self.iter().copied().map(Into::into)
    }
}

macro_rules! impl_tuple_binds {
    ($($name:ident),+) => {
        impl<$($name),+> InputBindings for ($($name,)+)
        where
            $($name: InputBindings),+
        {
            #[allow(non_snake_case)]
            fn iter_bindings(self) -> impl Iterator<Item = InputBind> {
                let ($($name,)+) = self;
                std::iter::empty()
                    $(.chain($name.iter_bindings()))+
            }
        }
    };
}

bevy::utils::all_tuples!(impl_tuple_binds, 1, 15, I);
