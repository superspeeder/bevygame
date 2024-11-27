#[macro_export]
macro_rules! configure_stated_system_set {
    ($app:ident, $set:expr, $state:expr) => {
        $app.configure_sets(Update, $set.run_if(in_state($state)));
        $app.configure_sets(FixedUpdate, $set.run_if(in_state($state)));
    };
}

#[macro_export]
macro_rules! marker {
    ($name: tt) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Component)]
        pub struct $name;
    };
}

#[macro_export]
macro_rules! flag_component {
    ($name: tt) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, Component)]
        pub struct $name(pub bool);
    };
}


#[macro_export]
macro_rules! flag_state {
    ($name: tt) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, States)]
        pub struct $name(pub bool);

        impl Into<bool> for $name {
            fn into(self) -> bool {
                self.0
            }
        }
    };
}
