pub const _BEGIN: &str = "\u{001b}";
pub const _END: &str = "\u{001b}[0m";

#[macro_export]
macro_rules! color {

    ($name:ident, $color:expr) => {

        /// A Wrapper struct generated by the color! macro that wraps 
        /// struct formatter output in terminal escape sequences.
        pub struct $name<T>(pub T);

        impl<T: core::fmt::Debug> core::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}{}{:?}{}", $crate::macros::_BEGIN, $color, self.0, $crate::macros::_END)
             }
        }

        impl<T: core::fmt::Display> core::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}{}{}{}", $crate::macros::_BEGIN, $color, self.0, $crate::macros::_END)
             }
        }

        impl<T> core::ops::Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> core::ops::DerefMut for $name<T> {
            //type Target = T;

            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
}
