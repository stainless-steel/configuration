use options::Options;
use std::path::Path;
use toml::{Array, Parser, Table, Value};

use {Node, Result, Tree};

/// The TOML format.
pub struct TOML;

impl TOML {
    /// Open a file.
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Tree> {
        use std::fs::File;
        use std::io::Read;

        let mut content = String::new();
        ok!(ok!(File::open(path)).read_to_string(&mut content));
        TOML::parse(&content)
    }

    /// Parse a text.
    pub fn parse(content: &str) -> Result<Tree> {
        let mut parser = Parser::new(content);
        match parser.parse() {
            Some(table) => Ok(Tree::from(try!(convert_table(table)))),
            _ => raise!("failed to parse ({})", collect_errors(&parser)),
        }
    }
}

fn convert_array(array: Array) -> Result<Vec<Node>> {
    let mut nodes = vec![];
    for value in array {
        if let Value::Table(inner) = value {
            nodes.push(try!(convert_table(inner)));
        } else {
            raise!("expected a table");
        }
    }
    Ok(nodes)
}

fn convert_table(mut table: Table) -> Result<Node> {
    let mut options = Options::new();

    for (name, _) in &table {
        options.set(name, 0);
    }
    for (name, value) in &mut options {
        match table.remove(name).unwrap() {
            Value::Array(inner) => value.set(try!(convert_array(inner))),
            Value::Boolean(inner) => value.set(inner),
            Value::Datetime(inner) => value.set(inner),
            Value::Float(inner) => value.set(inner),
            Value::Integer(inner) => value.set(inner),
            Value::String(inner) => value.set(inner),
            Value::Table(inner) => value.set(try!(convert_table(inner))),
        }
    }

    Ok(Node::from(options))
}

fn collect_errors(parser: &Parser) -> String {
    let mut errors = String::new();
    for error in parser.errors.iter() {
        if !errors.is_empty() {
            errors.push_str(", ");
        }
        errors.push_str(&format!("{}", error));
    }
    errors
}
