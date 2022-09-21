const BILLION: usize = 1_000_000_000;
const MILLION: usize = 1_000_000;
const THOUSAND: usize = 1_000;

const BLUE: &str = "#007ec6";
const GREY: &str = "#555555";

fn trim_and_float(amount: usize, size: usize) -> f64 {
    return (amount as f64) / (size as f64)
}

fn main() -> std::io::Result<()> {
    let mut stats = tokei::Language::new();
    let mut languages = tokei::Languages::new();

    languages.get_statistics(&["."], &[], &tokei::Config::default());
    for (_, language) in languages {
        stats += language;
    }

    println!(
        "Summary: Lines {lines} Code {code} Comments {comments} Blanks {blanks}",
        lines = stats.lines(),
        code = stats.code,
        comments = stats.comments,
        blanks = stats.blanks
    );

    let amount = stats.code;

    let amount = if amount >= BILLION {
        format!("{:.1}B", trim_and_float(amount, BILLION))
    } else if amount >= MILLION {
        format!("{:.1}M", trim_and_float(amount, MILLION))
    } else if amount >= THOUSAND {
        format!("{:.1}K", trim_and_float(amount, THOUSAND))
    } else {
        amount.to_string()
    };

    let badge = rsbadges::Badge {
        label_text: String::from("lines of code"),
        label_color: String::from(GREY),
        msg_text: amount,
        msg_color: String::from(BLUE),
        ..rsbadges::Badge::default()
    };

    let badge_style = rsbadges::Style::Flat(badge);

    let badge_svg = badge_style.generate_svg().unwrap();

    let mut out_dir = std::env::current_dir()?;
    out_dir.push("out");
    std::fs::create_dir(&out_dir);

    rsbadges::save_svg(&(out_dir.as_path().to_str().unwrap().to_owned() + "/badge.svg"), &badge_svg);

    Ok(())
}
