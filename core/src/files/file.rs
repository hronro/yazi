use std::{borrow::Cow, collections::BTreeMap, ffi::OsStr, fs::Metadata};

use anyhow::Result;
use shared::Url;
use tokio::fs;

#[derive(Clone, Debug)]
pub struct File {
	pub url:            Url,
	pub meta:           Metadata,
	pub length:         u64,
	pub(super) link_to: Option<Url>,
	pub is_link:        bool,
	pub is_hidden:      bool,
}

impl File {
	#[inline]
	pub async fn from(url: Url) -> Result<Self> {
		let meta = fs::metadata(&url).await?;
		Ok(Self::from_meta(url, meta).await)
	}

	pub async fn from_meta(url: Url, mut meta: Metadata) -> Self {
		let is_link = meta.is_symlink();
		let mut link_to = None;

		if is_link {
			meta = fs::metadata(&url).await.unwrap_or(meta);
			link_to = fs::read_link(&url).await.map(Url::from).ok();
		}

		let length = meta.len();
		let is_hidden = url.file_name().map(|s| s.to_string_lossy().starts_with('.')).unwrap_or(false);
		Self { url, meta, length, link_to, is_link, is_hidden }
	}

	#[inline]
	pub fn into_map(self) -> BTreeMap<Url, File> {
		let mut map = BTreeMap::new();
		map.insert(self.url.clone(), self);
		map
	}
}

impl File {
	// --- Url
	#[inline]
	pub fn url(&self) -> Url { self.url.clone() }

	#[inline]
	pub fn name(&self) -> Option<&OsStr> { self.url.file_name() }

	#[inline]
	pub fn name_display(&self) -> Option<Cow<str>> {
		self.url.file_name().map(|s| s.to_string_lossy())
	}

	#[inline]
	pub fn stem(&self) -> Option<&OsStr> { self.url.file_stem() }

	#[inline]
	pub fn parent(&self) -> Option<Url> { self.url.parent_url() }

	// --- Meta
	#[inline]
	pub fn is_file(&self) -> bool { self.meta.is_file() }

	#[inline]
	pub fn is_dir(&self) -> bool { self.meta.is_dir() }

	// --- Link to / Is link
	#[inline]
	pub fn link_to(&self) -> Option<&Url> { self.link_to.as_ref() }
}
