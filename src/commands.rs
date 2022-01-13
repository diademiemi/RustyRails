use chrono::{DateTime, NaiveTime, Utc};
use chrono_tz::Tz;
use rand::Rng;
use std::str::FromStr;
pub fn commands(command: String, content: String) -> String {
    let mut output = String::new();
    match command.as_str() {
        // 8ball command
        "8ball" => {
            // Initialise options
            let output_options = [
                "It is certain.",
                "It is decidedly so.",
                "Without a doubt.",
                "Yes - definitely.",
                "You may rely on it.",
                "As I see it, yes.",
                "Most likely.",
                "Outlook good.",
                "Yes.",
                "Signs point to yes.",
                "Reply hazy, try again.",
                "Ask again later.",
                "Better not tell you now.",
                "Cannot predict now.",
                "Concentrate and ask again.",
                "Don't count on it.",
                "My reply is no.",
                "My sources say no.",
                "Outlook not so good.",
                "Very doubtful."
            ];
            // Get random output
            output = format!("{}", output_options[rand::thread_rng().gen_range(0..output_options.len())])
        }
        // Minecraft stacks command
        // Converts an amount of items to stacks
        "mcstacks" => {
            // Get all the numbers in the message
            let items = content.chars().filter(|c| c.is_digit(10)).collect::<String>();
            if !items.is_empty() {
                if let Some(items) = items.parse::<i32>().ok() {
                    // Get the amount of stacks
                    let stacks: i32 = items / 64;
                    // Get the remainder
                    let left: i32 = items % 64;
                    // Format string
                    output = format!("{} item{} break{} down into {} stack{} with {} item{} left over", 
                    // Replace item count
                    items,
                    // Make items plural if needed
                    if items == 1 { "" } else { "s" },
                    // Make verb plural if needed
                    if items != 1 { "" } else { "s" },
                    // Replace stacks count
                    stacks,
                    // Make stacks plural if needed
                    if stacks == 1 { "" } else { "s" },
                    // Replace remainder
                    left,
                    // Make items plural if needed
                    if left == 1 { "" } else { "s" });
                }
            }
            if output.is_empty() {
                output = "Usage: <number of items>".to_string();
            }
        }
        // Minecraft items command
        // Converts amount of stacks (With optional decimal point) to amount of items
        "mcitems" => {
            // Get all the numbers (and periods) in the message
            let stacks = content.chars().filter(|c| c.is_digit(10) || c == &'.').collect::<String>();
            // Check if numbers were found
            if !stacks.is_empty() {
                // Try to parse into float
                if let Some(s) = stacks.parse::<f32>().ok() {
                    if s.is_finite() {
                        // Multiply by 32
                        let items: f32 = s * 64.0;
                        // Format string
                        output = format!("{} stack{} break{} down into {:.0} item{}",
                        // Replace stack count
                         s,
                        // Make stacks plural if needed
                        if s == 1.0 { "" } else { "s" },
                        // Make verb plural if needed
                        if s != 1.0 { "" } else { "s" },
                        // Replace item output
                        items,
                        // Make items plural if needed
                        if items == 1.0 { "" } else { "s"});
                    } else {
                        // If user inputs ridiculously high number
                        output = "I dunno lol".to_string();
                    }
                }
            }
            if output.is_empty() {
                output = "Usage: <number of stacks>".to_string();
            }
        }
        // If the command is temp
        "temp" => {
            // Get all the numbers (and periods) in the message
            let degrees = content.chars().filter(|c| c.is_digit(10) || c == &'.' || c == &'-').collect::<String>();
            // Search for a C or F
            let format = content.to_lowercase().chars().find(|c| c == &'c' || c == &'f');
            // If a format was found
            if let Some(f) = format {
                // If a number was found
                if !degrees.is_empty() {
                    if f == 'f' {
                        // Check if the number is finite
                        if let Some(fahrenheit) = degrees.parse::<f32>().ok() {
                            if fahrenheit.is_finite() {
                                // Perform the calculation
                                let result: f32 = (fahrenheit - 32.0) / 1.8;
                                // Set the output string
                                output = format!("{} in Fahrenheit is {} in Celsius.", fahrenheit, (result * 100.00).round() / 100.0);
                            } else {
                                // If user inputs ridiculously high number
                                output = "I dunno lol".to_string();
                            }
                        }
                    } else if f == 'c' {
                        if let Some(celsius) = degrees.parse::<f32>().ok() {
                            if celsius.is_finite() {
                                let result: f32 = (celsius * 1.8) + 32.0;
                                output = format!("{} in Celsius is {} in Fahrenheit.", celsius, (result * 100.00).round() / 100.0);
                            } else {
                                output = "I dunno lol".to_string();
                            }
                        }
                    }
                }
            }
            if output.is_empty() {
                output = "Usage: <Degrees> <C|F>".to_string();
            }
        }
        "timezone" => {
            // Get all characters matching 0-9 and :
            let mut time_str = content.split(' ').take(1).next().unwrap_or_default().to_uppercase()
            .chars().filter(|c| c.is_digit(10) || c == &':').collect::<String>();
            // Attempt to get the AM/PM string
            let meridiem_str = content.split(' ').take(1).next().unwrap_or_default().to_uppercase()
            .chars().filter(|c| c == &'A' || c == &'P' || c == &'M').collect::<String>();
            // Check if the string specifies minutes, if not, assume 00
            if !time_str.contains(":") {
                time_str.push_str(":00");
            }
            // Merge the time string and AM/PM (meridiem)
            time_str.push_str(&meridiem_str);
            // Attempt to get the first timezone
            let tz_str = content.split(' ').skip(1).next().unwrap_or_default().
            chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
            // Attempt to get a second timezone
            let to_tz_str = content.split(' ').skip(2).next().unwrap_or_default().
            chars().filter(|c| c == &'/' || c == &'_' || c.is_ascii_alphabetic()).collect::<String>();
            // Check if a timezone was found
            if !tz_str.is_empty() {
                // Check if timezone 1 can be parsed
                if let Ok(tz) = Tz::from_str(&tz_str) {
                    // Check if the time is in 12 hour format, if it is, convert it to 24 hours before proceeding
                    if let Ok(ntime) = NaiveTime::parse_from_str(&time_str, "%-I:%M%p") {
                        time_str = ntime.format("%H:%M").to_string();
                    }
                    // Attempt to parse the time as 24 hours
                    if let Ok(ntime) = NaiveTime::parse_from_str(&time_str, "%H:%M") {
                        // Get the time in the specified time zone with the current date in that timezone
                        let tz_today: DateTime<Tz> = Utc::today().with_timezone(&tz).and_time(ntime).unwrap();
                        // If a second timezone can be parsed, convert the two timezones and link to a time.is comparison table
                        if let Ok(to_tz) = Tz::from_str(&to_tz_str) {
                            output = format!("{} {} today in {} is {}, more at https://time.is/compare/{}_in_{}/{}",
                            ntime.format("%H:%M").to_string(), tz.name(), to_tz.name(), tz_today.with_timezone(&to_tz).format("%H:%M"), ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap(), to_tz.name().split('/').last().unwrap());
                        // If only one timezone was specified, use Discord's Unix Timestamp embed and link to a time.is table using an unspecified timezone
                        } else {
                            output = format!("{} {} today in your local timezone is <t:{}>, more at https://time.is/compare/{}_in_{}",
                            ntime.format("%H:%M").to_string(), tz.name(),tz_today.format("%s").to_string() , ntime.format("%H%M").to_string(), tz.name().split('/').last().unwrap());
                        }
                    }
                }
            }
            if output.is_empty() {
                output = "Usage: <HH:MM(AM/PM)> <TZ> (TZ)".to_string();
            }
        }
        _ => (),
    }
    output
}