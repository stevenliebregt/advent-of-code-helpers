use crate::line_iterator::{LineIterator, LineIteratorSettings};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::str::FromStr;

pub struct ParsingLineIterator<'a, T> {
    line_iterator: LineIterator<'a>,
    marker: PhantomData<T>,
}

impl<'a, T> ParsingLineIterator<'a, T> {
    pub fn from(input: &'a str) -> Self {
        Self {
            line_iterator: LineIterator::from(input),
            marker: Default::default(),
        }
    }

    pub fn from_settings(input: &'a str, settings: LineIteratorSettings) -> Self {
        Self {
            line_iterator: LineIterator::from_settings(input, settings),
            marker: Default::default(),
        }
    }
}

impl<'a, T> From<LineIterator<'a>> for ParsingLineIterator<'a, T> {
    fn from(line_iterator: LineIterator<'a>) -> Self {
        Self {
            line_iterator,
            marker: PhantomData::<T>::default(),
        }
    }
}

impl<'a, T> Iterator for ParsingLineIterator<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.line_iterator.next() {
            return Some(line.parse::<T>().unwrap());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_custom_structs() {
        #[derive(Debug, Eq, PartialEq)]
        enum OpType {
            Add,
            Multiply,
            Divide,
            Subtract,
        }

        impl FromStr for OpType {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "add" => Ok(OpType::Add),
                    "mul" => Ok(OpType::Multiply),
                    "div" => Ok(OpType::Divide),
                    "sub" => Ok(OpType::Subtract),
                    _ => Err(format!("Invalid op_type: {s}")),
                }
            }
        }

        #[derive(Debug, Eq, PartialEq)]
        struct Op {
            op_type: OpType,
            op_value: i32,
        }

        impl FromStr for Op {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (op_type_str, op_value_str) = s.split_once(' ').unwrap();

                Ok(Self {
                    op_type: op_type_str.parse().unwrap(),
                    op_value: op_value_str.parse().unwrap(),
                })
            }
        }

        let input = r#"add 10
mul 2
div 3
sub 12"#;

        let parsing_line_iterator = ParsingLineIterator::<Op>::from(input);

        let ops = parsing_line_iterator.collect::<Vec<_>>();

        assert_eq!(
            vec![
                Op {
                    op_type: OpType::Add,
                    op_value: 10,
                },
                Op {
                    op_type: OpType::Multiply,
                    op_value: 2,
                },
                Op {
                    op_type: OpType::Divide,
                    op_value: 3,
                },
                Op {
                    op_type: OpType::Subtract,
                    op_value: 12,
                }
            ],
            ops
        )
    }
}
