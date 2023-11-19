pub use overview::VersionOverview;
use {
	profile::{
		read_profile,
		ProfileEntry,
	},
	std::path::Path,
	tokio::fs,
};

mod overview;

pub async fn lookup_versions(root: &Path) -> Result<Vec<VersionOverview>, std::io::Error> {
	let versions_dir = root.join("versions");
	let mut versions_entries = fs::read_dir(&versions_dir).await?;

	let mut versions = Vec::<VersionOverview>::new();

	while let Some(entry) = versions_entries.next_entry().await? {
		let id = entry.file_name();
		let id_as_str = id.to_str().unwrap();

		let profiles = read_profile(root).await?.profiles;

		if let Some(ProfileEntry { icon, .. }) = profiles.get(id_as_str) {
			versions.push(VersionOverview {
				id: id_as_str.to_owned(),
				icon: icon.to_owned(),
			})
		} else {
			versions.push(VersionOverview {
				id: id_as_str.to_owned(),
				icon: None,
			})
		}
	}

	Ok(versions)
}