use log::error;
use tl::{parse, Bytes, Node, ParserOptions};

use tailwind_css::{CssInlineMode, TailwindBuilder};

use crate::{config::HtmlConfig, GlobalConfig, Result};

impl GlobalConfig {
    pub fn builder(&self) -> TailwindBuilder {
        TailwindBuilder::default()
    }
    pub fn compile_html(&self, input: &str, tw: &mut TailwindBuilder, mode: &CssInlineMode) -> Result<(String, String)> {
        let html = match mode {
            CssInlineMode::None => HtmlConfig::trace_all_class(input, tw)?,
            CssInlineMode::Inline => HtmlConfig::inline_all_class(input, tw)?,
            CssInlineMode::Scoped => HtmlConfig::scope_all_class(input, tw)?,
            CssInlineMode::DataKey => HtmlConfig::keyed_all_class(input, tw)?,
            CssInlineMode::DataValue => HtmlConfig::value_all_class(input, tw)?,
        };
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
}

impl HtmlConfig {
    pub fn trace_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            trace_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn inline_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            inline_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn scope_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            scope_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn keyed_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            key_class(node, tw);
        }
        Ok(dom.inner_html())
    }
    pub fn value_all_class(input: &str, tw: &mut TailwindBuilder) -> Result<String> {
        let mut dom = parse(input, ParserOptions::default())?;
        for node in dom.nodes_mut() {
            // ignore if any problem
            value_class(node, tw);
        }
        Ok(dom.inner_html())
    }
}

fn trace_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.trace(class.try_as_utf8_str()?) {
        Ok(c) => {
            class.set(c).ok()?;
        },
        Err(e) => error!("{}", e),
    }
    Some(())
}

fn inline_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    let mut style = Bytes::new();
    match tw.inline(class.try_as_utf8_str()?) {
        Ok((c, s)) => {
            class.set(c).ok()?;
            style.set(s).ok()?;
        },
        Err(e) => {
            error!("{}", e);
            return Some(());
        },
    };
    attributes.insert("style", Some(style));
    Some(())
}

fn scope_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    match tw.scope(class.try_as_utf8_str()?) {
        Ok((c1, c2)) => {
            class.set([c1, c2].join(" ")).ok()?;
        },
        Err(e) => {
            error!("{}", e);
        },
    };
    Some(())
}

fn key_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    let mut key = Bytes::new();
    match tw.data_key(class.try_as_utf8_str()?) {
        Ok((c, k)) => {
            class.set(c).ok()?;
            debug_assert!(k.len() == 12);
            key.set(format!("data-tw-{}", &k[1..12])).ok()?;
        },
        Err(e) => {
            error!("{}", e);
        },
    };
    attributes.insert::<_, &str>(key, None);
    Some(())
}

fn value_class(node: &mut Node, tw: &mut TailwindBuilder) -> Option<()> {
    let attributes = node.as_tag_mut()?.attributes_mut();
    let class = attributes.get_mut("class")??;
    let mut value = Bytes::new();
    match tw.data_value(class.try_as_utf8_str()?) {
        Ok((c, v)) => {
            class.set(c).ok()?;
            debug_assert!(v.len() == 12);
            value.set(&v[1..12]).ok()?;
        },
        Err(e) => {
            error!("{}", e);
        },
    };
    attributes.insert("data-tw", Some(value));
    Some(())
}