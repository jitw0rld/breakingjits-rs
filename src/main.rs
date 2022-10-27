use wallpaper;

const URL: &str = "http://71.72.237.156/breakingbad";

fn main() {
    wallpaper::set_from_url(URL).unwrap();
}
