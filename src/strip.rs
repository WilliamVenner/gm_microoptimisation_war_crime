//! Generate a data structure containing ranges of comments and strings for ignoring during optimisation passes.

use std::ops::Range;

#[derive(Default)]
pub(crate) struct StripTreeBuilder(Vec<Range<usize>>);
impl StripTreeBuilder {
	#[inline]
	pub(crate) fn insert(&mut self, range: Range<usize>) {
		self.0.push(range);
	}

	#[inline]
	pub(crate) fn build(self) -> StripTree {
		StripTree(self.0)
	}
}

pub(crate) struct StripTree(Vec<Range<usize>>);
impl StripTree {
	pub(crate) fn contains(&self, other: Range<usize>) -> bool {
		for range in &self.0 {
			if other.start >= range.start && other.end <= range.end {
				return true;
			}
		}
		false
	}
}

macro_rules! strips {
	{ $($name:ident => $re:literal),* } => {
		#[doc(hidden)]
		mod regex {
			#![allow(non_upper_case_globals)]
			magic_statics_mod! {
				$(pub(super) static ref $name: crate::Regex = crate::Regex::new($re).unwrap();)*
			}
		}

		$(
			#[doc(hidden)]
			fn $name(tree: &mut StripTreeBuilder, src: &[u8]) {
				for m in self::regex::$name.find_iter(src).map(|m| m.unwrap()) {
					tree.insert((m.start()..m.end()).into());
				}
			}
		)*

		pub(crate) fn generate(src: &[u8]) -> StripTree {
			let mut strip_tree = StripTreeBuilder::default();
			$(
				$name(&mut strip_tree, src);
			)*
			strip_tree.build()
		}

		#[inline]
		pub fn magic_static() {
			regex::magic_static()
		}
	};
}

strips! {
	comments => r#"--(?:\[(=*)\[[\S\s]+?\]\1\]|[\S\s]+?(?:\n|(?m:$)))"#,
	strings => r#"(?:("|')((?:\\\1|\\\\|.)*?)\1)|(?<!--)(?:\[(=*)\[([\s\S]*?)\]\3\])"#
}