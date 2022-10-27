use wallpaper;

const URL: &str = "http://ethix.tplinkdns.net/breakingbad";

fn main() {
    wallpaper::set_from_url(URL).unwrap();
}
