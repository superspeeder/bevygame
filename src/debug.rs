use crate::{flag_component, flag_state, marker};
use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ValidationSystemSet;

#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugDisplaySystemSet;

marker!(DebugMarker);
flag_component!(DebugEnabled);
flag_state!(DebugMode);

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        PostUpdate,
        ValidationSystemSet.run_if(|| cfg!(feature = "debugging")),
    );

    app.configure_sets(
        Update,
        DebugDisplaySystemSet.run_if(|| cfg!(feature = "debugging")),
    );

    app.add_plugins((debug_display::plugin, validation::plugin));
}

pub mod debug_display {
    use crate::debug::{DebugDisplaySystemSet, DebugEnabled, DebugMarker, DebugMode};
    use crate::marker;
    use bevy::prelude::*;

    marker!(DebugErrorMessage);

    pub(super) fn plugin(app: &mut App) {
        app.insert_state(DebugMode(cfg!(feature = "debugging")))
            .add_systems(
                Update,
                (manage_visibility1.run_if(in_state(DebugMode(true)))).in_set(DebugDisplaySystemSet),
            );
    }

    pub fn manage_visibility1(mut query: Query<(Mut<Visibility>, Has<DebugEnabled>), With<DebugMarker>>) {
        for (mut visibility, enabled) in query.iter_mut() {
            *visibility = if enabled { Visibility::Inherited } else { Visibility::Hidden };
        }
    }
}

pub mod validation {
    use bevy::prelude::*;

    #[derive(Debug, Eq, PartialEq, Clone, Event)]
    pub struct ValidationErrorEvent(String, ValidationCheck);

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    #[allow(dead_code)]
    pub enum ValidationCheck {
        ExactlyN(usize),
        AtLeastN(usize),
        AtMostN(usize),
        ComponentValidationError,
    }

    impl ValidationCheck {
        #[allow(dead_code)]
        pub fn make_message(&self, message: String) -> String {
            format!("{:?}: {}", self, message)
        }
    }

    #[allow(dead_code)]
    pub trait ComponentValidator<T: Component> {
        fn validate_component(value: &T) -> Result<(), String>;
    }

    pub fn plugin(app: &mut App) {
        app.add_event::<ValidationErrorEvent>();
    }

    #[allow(dead_code)]
    pub fn exactly_n<T: Component, const N: usize>(
        query: Query<Entity, With<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        let count = query.iter().count();
        if count != N {
            validation_error_state_writer.send(ValidationErrorEvent(format!("Wrong number of entities with the {} component (required exactly {:?}, found {:?})", std::any::type_name::<T>(), N, count), ValidationCheck::ExactlyN(count)));
        }
    }

    #[allow(dead_code)]
    pub fn at_least_n<T: Component, const N: usize>(
        query: Query<Entity, With<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        let count = query.iter().count();
        if count < N {
            validation_error_state_writer.send(ValidationErrorEvent(format!("Wrong number of entities with the {} component (required at least {:?}, found {:?})", std::any::type_name::<T>(), N, count), ValidationCheck::AtLeastN(count)));
        }
    }

    #[allow(dead_code)]
    pub fn at_most_n<T: Component, const N: usize>(
        query: Query<Entity, With<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        let count = query.iter().count();
        if count > N {
            validation_error_state_writer.send(ValidationErrorEvent(format!("Wrong number of entities with the {} component (required at most {:?}, found {:?})", std::any::type_name::<T>(), N, count), ValidationCheck::AtMostN(count)));
        }
    }

    //noinspection DuplicatedCode
    #[allow(dead_code)]
    pub fn component_validator<T: Component, V: ComponentValidator<T>>(
        query: Query<&T>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        for component in query.iter() {
            if let Err(msg) = V::validate_component(component) {
                validation_error_state_writer.send(ValidationErrorEvent(
                    msg,
                    ValidationCheck::ComponentValidationError,
                ));
            }
        }
    }

    //noinspection DuplicatedCode
    #[allow(dead_code)]
    pub fn conservative_component_validator<T: Component, V: ComponentValidator<T>>(
        query: Query<&T, Changed<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        for component in query.iter() {
            if let Err(msg) = V::validate_component(component) {
                validation_error_state_writer.send(ValidationErrorEvent(
                    msg,
                    ValidationCheck::ComponentValidationError,
                ));
            }
        }
    }
}
