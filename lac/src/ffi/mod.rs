#[cfg(feature = "use_bindgen")]
{
    pub mod bindings_gen;
    use bindings_gen::*;
}

#[cfg(not(feature = "use_bindgen"))]
{
    pub mod bindings;
    use bindings::*;
}

use std::os::raw::c_int;
