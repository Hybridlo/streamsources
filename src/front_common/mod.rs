mod scalable_wrapper;
mod options_util;
mod action_list;

pub mod predictions;
pub mod hypetrain;
pub mod animated_values;
pub(crate) mod components;

pub mod common_params;
pub mod transition_funcs;

pub use common_params::SourceColor;
pub use scalable_wrapper::scalable_wrapper;
pub use options_util::IntoWithLogin;
pub use action_list::use_action_list;

#[macro_export]
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub(crate) use enclose;