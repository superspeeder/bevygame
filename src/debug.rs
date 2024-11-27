use crate::{flag_component, flag_state, marker};
use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ValidationSystemSet;

#[derive(SystemSet, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugDisplaySystemSet;

marker!(DebugOnly);
flag_component!(DebugEnabled);
flag_state!(DebugMode)

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
    use bevy::prelude::*;
    use crate::debug::{DebugEnabled, DebugOnly};
    use crate::marker;

    marker!(DebugErrorMessage);

    pub(super) fn plugin(app: &mut App) {

    }

    pub fn manage_visibility1(mut query: Query<(Mut<Visibility>), Without<DebugEnabled>>) {
        for (mut visibility, _) in query.iter_mut() {}
    }

}

pub mod validation {
    use bevy::ecs::query::QueryFilter;
    use bevy::ecs::system::EntityCommands;
    use bevy::prelude::*;

    #[derive(Debug, Eq, PartialEq, Clone, Event)]
    pub struct ValidationErrorEvent(String, ValidationCheck);

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum ValidationCheck {
        ExactlyN(usize),
        AtLeastN(usize),
        AtMostN(usize),
        ComponentValidationError,
    }

    impl ValidationCheck {
        pub fn make_message(&self, message: String) -> String {
            format!("{:?}: {}", self, message)
        }
    }

    pub trait ComponentValidator<T: Component> {
        fn validate_component(value: &T) -> Result<(), String>;
    }

    pub trait ComponentWithCommandsValidator<T: Component> {
        fn validate_component(value: T, commands: EntityWorldMut) -> Result<(), String>;
    }

    pub fn plugin(app: &mut App) {
        app.add_event::<ValidationErrorEvent>();
    }

    pub fn exactly_n<T: Component, const N: usize>(
        query: Query<Entity, With<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        let count = query.iter().count();
        if count != N {
            validation_error_state_writer.send(ValidationErrorEvent(format!("Wrong number of entities with the {} component (required exactly {:?}, found {:?})", std::any::type_name::<T>(), N, count), ValidationCheck::ExactlyN(count)));
        }
    }

    pub fn at_least_n<T: Component, const N: usize>(
        query: Query<Entity, With<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        let count = query.iter().count();
        if count < N {
            validation_error_state_writer.send(ValidationErrorEvent(format!("Wrong number of entities with the {} component (required at least {:?}, found {:?})", std::any::type_name::<T>(), N, count), ValidationCheck::AtLeastN(count)));
        }
    }

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
    pub fn component_validator<T: Component, V: ComponentValidator<T>>(
        query: Query<&T>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        query.par_iter().for_each(|component| {
            if let Err(msg) = V::validate_component(component) {
                validation_error_state_writer.send(ValidationErrorEvent(
                    msg,
                    ValidationCheck::ComponentValidationError,
                ));
            }
        })
    }

    //noinspection DuplicatedCode
    pub fn component_validator_commands<T: Component, V: ComponentWithCommandsValidator<T>>(
        query: Query<(Entity, &T)>,
        mut par_commands: ParallelCommands,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        query.par_iter().for_each(|(entity, component)| {
            par_commands.command_scope(|mut c| c.entity(entity).add(|mut e: EntityWorldMut| {
                if let Err(msg) = V::validate_component(component, e) {
                    validation_error_state_writer.send(ValidationErrorEvent(
                        msg,
                        ValidationCheck::ComponentValidationError,
                    ));
                }
            }));
        });
    }

    //noinspection DuplicatedCode
    pub fn conservative_component_validator<T: Component, V: ComponentValidator<T>>(
        query: Query<&T, Changed<T>>,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        query.par_iter().for_each(|component| {
            if let Err(msg) = V::validate_component(component) {
                validation_error_state_writer.send(ValidationErrorEvent(
                    msg,
                    ValidationCheck::ComponentValidationError,
                ));
            }
        })
    }

    //noinspection DuplicatedCode
    pub fn conservative_component_validator_commands<T: Component, V: ComponentWithCommandsValidator<T>>(
        query: Query<(Entity, &T), Changed<T>>,
        mut par_commands: ParallelCommands,
        mut validation_error_state_writer: EventWriter<ValidationErrorEvent>,
    ) {
        query.par_iter().for_each(|(entity, component)| {
            par_commands.command_scope(|mut c| c.entity(entity).add(|mut e: EntityWorldMut| {
                if let Err(msg) = V::validate_component(component, e) {
                    validation_error_state_writer.send(ValidationErrorEvent(
                        msg,
                        ValidationCheck::ComponentValidationError,
                    ));
                }
            }));
        });
    }
}
