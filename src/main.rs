use clap::Parser;
use libnotify_rs::{
    api::notification::{Timeout, Urgency},
    builder::NotificationBuilder,
    context::LibnotifyContext,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Notification summary
    summary: String,
    /// Notification body
    body: Option<String>,
    #[arg(short, long)]
    app_name: Option<String>,
    /// Path to an icon or just icon name
    #[arg(long)]
    app_icon: Option<String>,
    /// Path to an icon or just icon name
    #[arg(long, short)]
    icon: Option<String>,
    #[arg(long, short)]
    urgency: Option<Urgency>,
    /// Time in miliseconds, for the notification to appear on screen. Not all implementations use
    /// this parameter. Don't set for default, set -1 for never.
    #[arg(long, short)]
    expire_time: Option<i64>,
    /// Specifies the notification category
    #[arg(long, short)]
    category: Option<String>,
    /* Missing fields:
     * - action
     * - selected-action-fd=FILE_DESCRIPTOR
     * - activation-token-fd=FILE_DESCRIPTOR
     * - app_icon
     * - hint
     * - print-id
     * - replace-id
     * - wait
     * - transient
     */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let timeout: Timeout = match args.expire_time {
        None => Timeout::Default,
        Some(-1) => Timeout::Never,
        Some(x) => Timeout::Custom(x.try_into().unwrap_or(1000 * 5)),
    };

    let ctx = LibnotifyContext::new(&args.app_name.as_ref().unwrap_or(&"notify-send".to_string()))?;
    let notification = NotificationBuilder::new(
        &args.summary,
        args.body.as_deref(),
        args.icon.as_deref(),
        &ctx,
    )?
    .set_app_name(args.app_name.as_deref())?
    .set_app_icon(args.app_icon.as_deref())?
    .set_urgency(args.urgency)
    .set_timeout(timeout)
    .set_category(args.category.as_ref().map(|category| category.as_str()))?;

    notification.show()?;

    Ok(())
}
