pub mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    mod inside {
        pub fn inner_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

// but the second privacy rule states that the try_me function 
// is allowed to access the outermost module because outermost is in the current 
// (root) module, as is try_me.