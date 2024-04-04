use std::env;

use oxc_ast::ast::Program;
use serde::Deserialize;

use crate::options::{default_as_true, JsxOptions};
use crate::preset_plugin::{BoxedTransformation, Transformation};

#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ReactOptions {
    /// https://babeljs.io/docs/babel-preset-react#development
    #[serde(default = "default_for_development")]
    pub development: bool,

    /// https://babeljs.io/docs/babel-plugin-transform-react-display-name
    pub display_name: bool,

    /// https://babeljs.io/docs/babel-preset-react#pure
    #[serde(default = "default_as_true")]
    pub pure: bool,

    /// https://babeljs.io/docs/babel-preset-react#runtime
    pub runtime: ReactJsxRuntime,

    /// https://babeljs.io/docs/babel-plugin-transform-react-jsx-self
    pub self_prop: bool,

    /// https://babeljs.io/docs/babel-plugin-transform-react-jsx-source
    pub source_prop: bool,

    /// https://babeljs.io/docs/babel-preset-react#throwifnamespace
    #[serde(default = "default_as_true")]
    pub throw_if_namespace: bool,
}

fn default_for_development() -> bool {
    env::var("NODE_ENV").is_ok_and(|var| var != "production")
}

impl Default for ReactOptions {
    fn default() -> Self {
        Self {
            development: default_for_development(),
            display_name: false,
            pure: default_as_true(),
            runtime: ReactJsxRuntime::Automatic,
            self_prop: false,
            source_prop: false,
            throw_if_namespace: default_as_true(),
        }
    }
}

/// Decides which runtime to use.
///
/// Auto imports the functions that JSX transpiles to.
/// classic does not automatic import anything.
#[derive(Debug, Default, Clone, Deserialize)]
pub enum ReactJsxRuntime {
    Classic,
    /// The default runtime is switched to automatic in Babel 8.
    #[default]
    Automatic,
}

pub struct React {
    jsx: JsxOptions,
    options: ReactOptions,
    plugins: Vec<BoxedTransformation>,
}

impl React {
    pub fn new(options: ReactOptions, jsx: JsxOptions) -> Self {
        Self { options, jsx, plugins: vec![] }
    }
}

impl Transformation for React {
    fn transform<'a>(&mut self, program: &mut Program<'a>) {
        for plugin in &mut self.plugins {
            plugin.transform(program);
        }
    }
}
