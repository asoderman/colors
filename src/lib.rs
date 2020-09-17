#![cfg_attr(feature = "no_std", no_std)]

pub mod ansi;

#[macro_use]
mod macros;

#[allow(unused_imports)] // Compiler is thinking #[macro_use] is unused
#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(test)]
mod tests {
    use crate::ansi;

    /// Test the Red struct created by the macro using the debug formatter.
    /// Notable difference from the Display version: The double quotes are
    /// included in the output to indicate we are dealing with strings.
     #[test]
    fn test_color_debug() {

        assert_eq!(format!("{:?}", ansi::Red("Hello world")), "\u{001b}[31m\"Hello world\"\u{001b}[0m");
    }

    /// Test the Red struct created by the macro.
    #[test]
    fn test_color_display() {

        assert_eq!(format!("{}", ansi::Red("Hello world")), 
                   "\u{001b}[31mHello world\u{001b}[0m");

    }

    /// Test if the user is able to create custom colors using a 
    /// 16 Color
    #[test]
    fn test_custom_color_16() {

        color!(MyGreen, "[32;1m");

        let test_value = MyGreen("This is (bright) green");

        #[cfg(feature="std")]
        println!("{}", test_value);

        assert_eq!(format!("{}",test_value), 
                   "\u{001b}[32;1mThis is (bright) green\u{001b}[0m");
    }

    /// Test if the user is able to create custom colors using a 
    /// 256 Color
    #[test]
    fn test_custom_color_256() {

        color!(MyGreen, "[32;1m");

        let test_value = MyGreen("This is (bright) green");

        #[cfg(feature="std")]
        println!("{}", test_value);

        assert_eq!(format!("{}",test_value), 
                   "\u{001b}[32;1mThis is (bright) green\u{001b}[0m");
    }

    /// Tests if the struct is still able to properly deref.
    #[test]
    fn test_deref() {

        let red_string = ansi::Red(100);
        let result = 50 + *red_string;
        assert_eq!(150, result);
    }

    #[test]
    fn test_deref_mut() {
        let mut blue_string = ansi::Blue(100);

        *blue_string = 150;

        // Assert inner value was mutated
        assert_eq!(150, *blue_string);
        // Assert we maintained our color information
        assert_eq!(format!("{:?}", blue_string), 
                   "\u{001b}[34m150\u{001b}[0m");
    }

}
