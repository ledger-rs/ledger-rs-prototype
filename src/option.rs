use crate::Kind;

/**
 * option.cc
 *
 * Processes command arguments and options.
 */

/// Process arguments?
/// returns (commands, options)
pub fn process_arguments(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut options: Vec<String> = vec![];
    let mut commands: Vec<String> = vec![];

    // iterate through the list
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if !arg.starts_with('-') {
            // otherwise return
            commands.push(arg.to_owned());
            continue;
        }

        // otherwise, it's an argument
        // if an item contains "-f", it is a real argument

        // long option
        if arg.starts_with("--") {
            // long argument
            if arg.len() == 2 {
                // it's a --, ending options processing
                todo!("handle this case?")
            } else if arg.len() == 1 {
                panic!("illegar option {}", arg)
            }

            // todo: check if there is '=' contained
            // else
            let option_name = &arg[2..];

            // TODO: find_option(option_name);

            // TODO: get argument value

            // TODO: process_option();

            todo!("complete")
        } else {
            // single-char option

            let mut option_queue = vec![];

            // iterate through all characters and identify options,
            for (i, c) in arg.char_indices() {
                if i == 0 {
                    // skipping the first ('-').
                    continue;
                }

                // check for a valid option and if it requires an argument?
                // Also links to a handler.
                // TODO: option = find_option(c);

                let mut option = String::from('-');
                option.push(c);

                // add option to the option queue
                option_queue.push(option);
            }

            // todo: for each option in option_queue (?)
            // multiple arguments are possible after "-".
            // The values come after the options.

            // get the option argument(s).
            // Iterate through option_queue and retrieve the value if required.
            for option in option_queue {
                // todo: there needs to be an indicator if the option requires a value.
                // if requires_value &&
                if let Some(value) = iter.next() {
                    // let mut whence = String::from("-");
                    // whence.push(arg.chars().nth(0).unwrap());

                    // TODO: check for validity, etc.
                    // process_option(whence, Some(value.to_owned()));
                    // options.push(whence);

                    // for now, just add
                    options.push(option);
                    options.push(value.to_owned());
                } else {
                    panic!("Missing option argument for {}", arg);
                }
            }
        }
    }

    (commands, options)
}

/// Searches through scopes for the option with the given letter.
/// Then links to a handler function(?).
fn find_option(letter: char) {
    let mut name = String::from(letter);
    name.push('_');

    // lookup first checks Session
    session_lookup(crate::Kind::OPTION, &name);

    todo!()
}

/// find_option() from global.cc
fn lookup_option_global(kind: Kind, letter: char) {
    match kind {
        Kind::PRECOMMAND => {
            // p => push, pop
        }
        _ => todo!(),
    }

    // adhiostv
    match letter {
        's' => todo!("script"),
        't' => todo!("trace"),
        _ => todo!("other chars"),
    }

    todo!()
}

fn process_option(whence: String, value: Option<String>) {
    let mut args = vec![];

    // add the argument and the value to a collection
    args.push(whence);

    match value {
        Some(val) => args.push(val),
        None => (),
    }

    // TODO: check for validity
    // if wants_arg ...
    // there have to be 2 args.
}

/// Lookup options for session
fn session_lookup(kind: Kind, name: &str) {
    let option = name.chars().nth(0).unwrap();

    match kind {
        Kind::FUNCTION => todo!(),
        Kind::OPTION => {
            // handler =
            session_lookup_option(option)
            // TODO: make_opt_handler(Session, handler)
        }
        _ => todo!(),
    }
}

/// Searches for a short-version option. i.e. -f for file
fn session_lookup_option(option: char) {
    match option {
        'Q' => todo!(),
        'Z' => todo!(),
        'c' => todo!(),
        'd' => todo!(),
        'e' => todo!(),
        'f' => {
            // OPT_(file_)
            todo!("option file_")
        }
        'i' => todo!(),
        'l' => todo!(),
        'm' => todo!(),
        'n' => todo!(),
        'p' => todo!(),
        'r' => todo!(),
        's' => todo!(),
        't' => todo!(),
        'v' => todo!(),
        _ => todo!("return NULL"),
    }
}

/// Lookup options for reports
fn lookup_report(kind: Kind, name: &str) {
    let letter: char = name.chars().nth(0).unwrap();

    match kind {
        Kind::FUNCTION => {
            todo!()
        }
        Kind::COMMAND => {
            match letter {
                'a' => {
                    if name == "accounts" {
                        todo!("accounts")
                        // POSTS_REPORTER(report_accounts)
                    }
                }
                'b' => {
                    // FORMATTED_ACCOUNTS_REPORTER(balance_format_)
                    todo!("balance")
                    // or budget
                }

                // cdel
                'p' => {
                    // print,
                    // POSTS_REPORTER(print_xacts)

                    // prices,
                    // pricedb,
                    // FORMATTED_COMMODITIES_REPORTER(pricedb_format_)

                    // pricemap,
                    // report_t::pricemap_command

                    // payees
                    // POSTS_REPORTER(report_payees)
                }
                'r' => {
                    // r, reg, register
                    // FORMATTED_POSTS_REPORTER(register_format_)

                    // reload
                    // report_t::reload_command

                    todo!("register")
                }

                // stx
                _ => todo!("the rest"),
            }
        }
        Kind::PRECOMMAND => {
            match letter {
                'a' => {
                    todo!("args")
                    // WRAP_FUNCTOR(query_command)
                }
                // efgpqst
                _ => todo!("handle"),
            }
            todo!("pre-command")
        }
        _ => todo!("handle"),
    }

    todo!("go through the report options")
}

fn lookup_option_report(letter: char) {
    // t:
    // amount, tail, total, total_data, truncate, total_width, time_report

    match letter {
        // %ABCDEFGHIJLMOPRSTUVWXYabcdefghijlmnopqrstuvwy
        'G' => todo!("gain"),     // OPT_CH(gain)
        'S' => todo!("sort"),     // OPT_CH(sort_)
        'X' => todo!("exchange"), // OPT_CH(exchange_)
        'a' => {
            // OPT(abbrev_len_);
            // else OPT_(account_);
            // else OPT(actual);
            // else OPT(add_budget);
            // else OPT(amount_);
            // else OPT(amount_data);
            // else OPT_ALT(primary_date, actual_dates);
            // else OPT(anon);
            // else OPT_ALT(color, ansi);
            // else OPT(auto_match);
            // else OPT(aux_date);
            // else OPT(average);
            // else OPT(account_width_);
            // else OPT(amount_width_);
            // else OPT(average_lot_prices);
            todo!()
        }
        _ => todo!("the rest"),
    }
}

#[cfg(test)]
mod tests {
    use shell_words::split;

    use crate::option::process_arguments;

    #[test]
    fn test_process_arguments() {
        let args = split("accounts -f basic.ledger").unwrap();

        let (commands, options) = process_arguments(args);

        assert_eq!(1, commands.len());
        assert_eq!("accounts", commands[0]);

        // options
        assert_eq!(2, options.len());
        assert_eq!("-f", options[0]);
        assert_eq!("basic.ledger", options[1]);
    }

    #[test]
    fn test_process_multiple_arguments() {
        let args = split("cmd -ab value_a value_b").unwrap();

        let (commands, options) = process_arguments(args);

        assert_eq!(1, commands.len());
        assert_eq!("cmd", commands[0]);

        // options
        assert_eq!(4, options.len());

        assert_eq!("-a", options[0]);
        assert_eq!("value_a", options[1]);

        assert_eq!("-b", options[2]);
        assert_eq!("value_b", options[3]);
    }

    #[test]
    fn test_multiple_commands() {
        let args: Vec<String> = shell_words::split("accounts b -f tests/minimal.ledger").unwrap();

        let (commands, options) = process_arguments(args);

        assert_eq!(2, commands.len());
        assert_eq!("accounts", commands[0]);
        assert_eq!("b", commands[1]);
    }
}
