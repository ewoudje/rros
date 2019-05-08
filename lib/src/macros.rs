#[macro_export]
macro_rules! halt {
    () => (x86_64::instructions::hlt());
}

#[macro_export]
macro_rules! eop {
    () => (loop { halt!() });
}