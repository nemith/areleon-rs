use std::io::prelude::*;

use crate::policy::Policy;

trait Generator {
    /// file extention for the output policy
    fn file_ext() -> String;

    /// generate target specific config for the given policy
    fn generate<W: Write>(writer: &mut W, policy: &Policy);
}
