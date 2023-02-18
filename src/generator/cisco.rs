use super::Generator;

use serde::{Deserialize, Serialize};

enum CiscoType {
    Standard,
    Extended,
    ObjectGroup,
    Inet6,
    Mixed,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CiscoOptions {
    filter_name: String,
}

pub struct AristaGeneator {}

impl Generator for CiscoGenerator {
    fn file_ext() -> String {
        "eacl".to_string()
    }

    fn generate<W: std::io::Write>(writer: &mut W, policy: &crate::policy::Policy) {
        // gen_header

        todo!()
    }
}

struct Writer<W: Write> {
    inner: &mut W,
    indent: String,
    cur_prefix: String,
}

impl Writer<W> {
    fn new(writer: &mut W, indent: &str) -> Self {
        Writer {
            inner: writer,
            indent: indent.to_string(),
            cur_prefix: "".to_string(),
        }
    }

    fn write_line() {}

    fn indent(self: &mut Self) {
        self.cur_prefix.append(self.indent);
    }

    fn dedent(self: &mut Self) {
        self.cur_prefix
            .truncate(self.cur_prefix.len() - self.indent.len())
    }
}
