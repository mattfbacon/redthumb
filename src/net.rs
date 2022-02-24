use reqwest::blocking::get;
use serde::Deserialize;
use std::io::Write;

pub fn generate(font: &str, post_id: &str, output: &mut dyn Write) -> anyhow::Result<()> {
	eprintln!("Making post info request");
	let post_info: serde_json::Value = serde_json::from_reader(get(format!("https://www.reddit.com/comments/{}/.json", post_id))?.error_for_status()?)?;
	fn unwrap_post_info(post_info: &serde_json::Value) -> Option<&serde_json::Value> {
		post_info.get(0)?.get("data")?.get("children")?.get(0)?.get("data")
	}
	let PostInfo { title, subreddit, subreddit_name_prefixed } = PostInfo::deserialize(unwrap_post_info(&post_info).ok_or_else(|| anyhow::anyhow!("Post info response missing required fields"))?)?;

	eprintln!("Making icon request");
	let icon: serde_json::Value = serde_json::from_reader(get(format!("https://www.reddit.com/r/{}/about.json", subreddit))?.error_for_status()?)?;
	fn unwrap_icon(icon: &serde_json::Value) -> Option<&serde_json::Value> {
		icon.get("data")?.get("icon_img")
	}
	let icon = match unwrap_icon(&icon) {
		Some(icon) => Some(String::deserialize(icon)?),
		None => None,
	};
	let mut icon = match icon {
		Some(icon) if !icon.is_empty() => Some({
			eprintln!("Making icon image data request");
			get(icon)?
		}),
		_ => None,
	};
	let icon = icon.as_mut().map(|icon| icon as &mut dyn std::io::Read);

	eprintln!("Generating image");
	crate::image::generate(font, &title, &subreddit_name_prefixed, icon, output)
}

#[derive(Deserialize)]
struct PostInfo {
	title: String,
	subreddit: String,
	subreddit_name_prefixed: String,
}
