use cairo::Context;
use std::io::{Read, Write};

const REDDIT_LOGO: &[u8; 22323] = core::include_bytes!("reddit.png");

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const PADDING: f64 = 50.0;
const ICON_SIZE: f64 = 800.0;

pub fn generate(font: &str, title: &str, subreddit: &str, icon: Option<&mut dyn Read>, mut output: &mut dyn Write) -> anyhow::Result<()> {
	let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, WIDTH, HEIGHT).expect("Creating Cairo surface");
	{
		let drawer = Drawer::new(&surface, font);
		drawer.draw_background();
		drawer.draw_subreddit(subreddit);
		drawer.draw_icon(icon.unwrap_or(&mut REDDIT_LOGO.as_slice()));
		drawer.draw_title(title);
	}
	surface.write_to_png(&mut output)?;
	Ok(())
}

struct Drawer<'a> {
	ctx: Context,
	font: &'a str,
}

impl<'a> Drawer<'a> {
	fn new(surface: &cairo::Surface, font: &'a str) -> Self {
		let ctx = Context::new(surface).expect("Creating Cairo context");
		ctx.set_antialias(cairo::Antialias::Best);
		Self { ctx, font }
	}

	fn print_text(&self, size: f64, text: &str, bounds: Option<(f64, f64)>) {
		let mut description = pango::FontDescription::new();
		description.set_family(self.font);
		description.set_weight(pango::Weight::Normal);
		description.set_absolute_size(pango::SCALE as f64 * size);

		let layout = pangocairo::create_layout(&self.ctx).expect("Creating layout");
		layout.set_font_description(Some(&description));
		layout.set_text(text);

		if let Some((width, height)) = bounds {
			layout.set_width((width * pango::SCALE as f64) as i32);
			layout.set_wrap(pango::WrapMode::WordChar);
			layout.set_height((height * pango::SCALE as f64) as i32);
			layout.set_ellipsize(pango::EllipsizeMode::End);
		}

		pangocairo::show_layout(&self.ctx, &layout);
	}

	fn draw_background(&self) {
		const START_COLOR: f64 = 0.102;
		const END_COLOR: f64 = 0.157;
		// zero width = vertical gradient
		let gradient = cairo::LinearGradient::new(0.0, 0.0, 0.0, HEIGHT as f64);
		gradient.add_color_stop_rgb(0.0, START_COLOR, START_COLOR, START_COLOR);
		gradient.add_color_stop_rgb(1.0, END_COLOR, END_COLOR, END_COLOR);
		self.ctx.set_source(&gradient).expect("Setting gradient as background");
		self.ctx.rectangle(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
		self.ctx.fill().expect("Filling background");
	}

	fn draw_subreddit(&self, subreddit: &str) {
		self.ctx.set_source_rgb(1.0, 0.12, 0.00);
		self.ctx.move_to(PADDING as f64, 50.0);
		self.print_text(60.0, subreddit, Some((WIDTH as f64 - ICON_SIZE - PADDING * 2.0, 60.0)));
	}

	fn draw_icon(&self, mut icon: &mut dyn Read) {
		// Cairo basically does everything matrix-related in reverse, so a scale less than 1 enlarges, translations are negative, and transformations are applied in reverse. Beware.
		let icon_surface = cairo::ImageSurface::create_from_png(&mut icon).expect("Creating icon surface");
		let pattern = cairo::SurfacePattern::create(&icon_surface);

		// Bound it within its area
		let icon_scale = f64::max((icon_surface.width() as f64) / ICON_SIZE, (icon_surface.height() as f64) / (HEIGHT as f64 - PADDING * 2.0));
		pattern.set_matrix({
			let mut matrix = pattern.matrix();
			matrix.scale(icon_scale, icon_scale);
			let x = WIDTH as f64 - PADDING - (ICON_SIZE / 2.0) - (icon_surface.width() as f64 / icon_scale / 2.0);
			let y = (HEIGHT as f64 - (icon_surface.height() as f64 / icon_scale)) / 2.0;
			matrix.translate(-x, -y);
			matrix
		});

		self.ctx.set_source(&pattern).expect("Setting icon as source pattern");
		self.ctx.paint().expect("Painting icon");
	}

	fn draw_title(&self, title: &str) {
		self.ctx.set_source_rgb(1.0, 1.0, 1.0);
		self.ctx.move_to(PADDING as f64, 150.0);
		self.print_text(96.0, title, Some((WIDTH as f64 - ICON_SIZE - PADDING * 3.0, HEIGHT as f64 - 150.0 - PADDING)))
	}
}
