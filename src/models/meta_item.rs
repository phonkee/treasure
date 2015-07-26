use syntax::ast::{Attribute,Lit,MetaItem_,MetaList,MetaNameValue,MetaWord};
use syntax::attr::AttrMetaMethods;

#[derive(Debug)]
pub enum MetaItemError {
	UnknownMetaItem,
	UnsupportedMetaItem,
}

pub trait MetaItemWalker {
	fn set_meta_item(&mut self, name:&String, item:&MetaItem_) -> Result<(), MetaItemError>;
}

fn is_valid_meta_item(_item:&MetaItem_) -> bool {
	true
}

// Walks items
pub fn walk_meta_items<F>(attrs:&Vec<Attribute>,
					name:&String,
					closure: F) -> Result<(), MetaItemError>
					where F: Fn(&String, &MetaItem_) -> Result<(), MetaItemError> {

	for attr in attrs.iter() {
		let meta_item = &attr.node.value.node;
		let nn = match *meta_item {
			MetaWord(ref n) => n.to_string(),
			MetaList(ref n, _) => n.to_string(),
			_ => return Err(MetaItemError::UnsupportedMetaItem)
		};
		if &nn == name {
			match *meta_item {
				MetaList(_, ref attrs) => {
					for a in attrs.iter() {

						let mn = a.name();

						match closure(&mn.clone().to_string(), &a.node) {
							Err(e) => return Err(e),
							_ => (),
						}
					}
				},
				_ => (),
			}
		}
	}

	Ok(())
}
