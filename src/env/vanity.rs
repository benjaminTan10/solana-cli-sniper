use std::{
    collections::HashSet,
    error,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Instant,
};

use bip39::Language;
use clap::{value_parser, App, Arg, ArgMatches, Command};
use rpassword::prompt_password;
use solana_sdk::{
    derivation_path::DerivationPath,
    signature::{
        write_keypair_file, Keypair,
    },
    signer::Signer,
};

use super::input::{confirmation, get_text_input};

mod smallest_length_44_public_key {
    use solana_sdk::{pubkey, pubkey::Pubkey};

    pub(super) static PUBKEY: Pubkey = pubkey!("21111111111111111111111111111111111111111111");

    #[test]
    fn assert_length() {
        use crate::env::vanity::smallest_length_44_public_key;
        assert_eq!(smallest_length_44_public_key::PUBKEY.to_string().len(), 44);
    }
}

pub async fn vanity_main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("Vanity")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Does awesome things")
        .subcommand(
            Command::new("grind")
                .about("Grind for vanity keypairs")
                .disable_version_flag(true)
                .arg(
                    Arg::new("ignore_case")
                        .long("ignore-case")
                        .help("Performs case insensitive matches"),
                )
                .arg(
                    Arg::new("starts_with")
                        .long("starts-with")
                        .value_name("PREFIX:COUNT")
                        .number_of_values(1)
                        .takes_value(true)
                        .multiple_occurrences(true)
                        .multiple_values(true)
                        .help("Saves specified number of keypairs whos public key starts with the indicated prefix\nExample: --starts-with sol:4\nPREFIX type is Base58\nCOUNT type is u64"),
                )
                .arg(
                    Arg::new("ends_with")
                        .long("ends-with")
                        .value_name("SUFFIX:COUNT")
                        .number_of_values(1)
                        .takes_value(true)
                        .multiple_occurrences(true)
                        .multiple_values(true)
                        .help("Saves specified number of keypairs whos public key ends with the indicated suffix\nExample: --ends-with ana:4\nSUFFIX type is Base58\nCOUNT type is u64"),
                )
                .arg(
                    Arg::new("starts_and_ends_with")
                        .long("starts-and-ends-with")
                        .value_name("PREFIX:SUFFIX:COUNT")
                        .number_of_values(1)
                        .takes_value(true)
                        .multiple_occurrences(true)
                        .multiple_values(true)
                        .help("Saves specified number of keypairs whos public key starts and ends with the indicated prefix and suffix\nExample: --starts-and-ends-with sol:ana:4\nPREFIX and SUFFIX type is Base58\nCOUNT type is u64"),
                )
                .arg(
                    Arg::new("num_threads")
                        .long("num-threads")
                        .value_name("NUMBER")
                        .takes_value(true)
                        .value_parser(value_parser!(usize))
                        .default_value("1")
                        .help("Specify the number of grind threads"),
                )
                .arg(
                    Arg::new("use_mnemonic")
                        .long("use-mnemonic")
                        .help("Generate using a mnemonic key phrase. Expect a significant slowdown in this mode"),
                )
                .arg(
                    Arg::new("derivation_path")
                        .long("derivation-path")
                        .takes_value(true)
                        .requires("use_mnemonic")
                        .help("Path for derivation, required if using mnemonic"),
                )
                .arg(
                    Arg::new("no_outfile")
                        .long("no-outfile")
                        .help("Do not output results to file"),
                )
        )
        .get_matches();

    // Determine which subcommand was used
    let subcommand = matches.subcommand_name().unwrap_or("");
    let matches = matches.subcommand_matches(subcommand).unwrap_or(&matches);

    let check = confirmation().await?;

    let ignore_case = matches.is_present("ignore_case");
    let mut starts_with_args: HashSet<String> = HashSet::new();
    let mut ends_with_args: HashSet<String> = HashSet::new();

    if check {
        let input = get_text_input("Enter prefix", "Input Starting Chars for Search").await;
        for s in input.split_whitespace() {
            if s.contains(':') {
                starts_with_args.insert(if ignore_case {
                    s.to_lowercase()
                } else {
                    s.to_string()
                });
            } else {
                eprintln!("Invalid format for starts_with_args: {}", s);
            }
        }
    } else {
        let input = get_text_input("Enter suffix", "Input Ending Chars for Search").await;
        for s in input.split_whitespace() {
            if s.contains(':') {
                ends_with_args.insert(if ignore_case {
                    s.to_lowercase()
                } else {
                    s.to_string()
                });
            } else {
                eprintln!("Invalid format for ends_with_args: {}", s);
            }
        }
    }

    if starts_with_args.is_empty() && ends_with_args.is_empty() {
        return Err(
            "Error: No keypair search criteria provided (--starts-with or --ends-with or --starts-and-ends-with)".into()
        );
    }

    let cpus = num_cpus::get();

    let num_threads: usize = matches.value_of_t("num_threads").unwrap_or(cpus);

    let grind_matches =
        grind_parse_args(ignore_case, starts_with_args, ends_with_args, num_threads);

    let no_outfile = matches.is_present("no_outfile");

    // The vast majority of base58 encoded public keys have length 44, but
    // these only encapsulate prefixes 1-9 and A-H.  If the user is searching
    // for a keypair that starts with a prefix of J-Z or a-z, then there is no
    // reason to waste time searching for a keypair that will never match
    let skip_len_44_pubkeys = grind_matches
        .iter()
        .map(|g| {
            let target_key = if ignore_case {
                g.starts.to_ascii_uppercase()
            } else {
                g.starts.clone()
            };
            let target_key =
                target_key + &(0..44 - g.starts.len()).map(|_| "1").collect::<String>();
            bs58::decode(target_key).into_vec()
        })
        .filter_map(|s| s.ok())
        .all(|s| s.len() > 32);

    let grind_matches_thread_safe = Arc::new(grind_matches);
    let attempts = Arc::new(AtomicU64::new(1));
    let found = Arc::new(AtomicU64::new(0));
    let start = Instant::now();
    let done = Arc::new(AtomicBool::new(false));

    let thread_handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let done = done.clone();
            let attempts = attempts.clone();
            let found = found.clone();
            let grind_matches_thread_safe = grind_matches_thread_safe.clone();

            thread::spawn(move || loop {
                if done.load(Ordering::Relaxed) {
                    break;
                }
                let attempts = attempts.fetch_add(1, Ordering::Relaxed);
                if attempts % 1_000_000 == 0 {
                    println!(
                        "Searched {} keypairs in {}s. {} matches found.",
                        attempts,
                        start.elapsed().as_secs(),
                        found.load(Ordering::Relaxed),
                    );
                }
                let (keypair, phrase) = (Keypair::new(), "".to_string());
                // Skip keypairs that will never match the user specified prefix
                if skip_len_44_pubkeys && keypair.pubkey() >= smallest_length_44_public_key::PUBKEY
                {
                    continue;
                }
                let mut pubkey = bs58::encode(keypair.pubkey()).into_string();
                if ignore_case {
                    pubkey = pubkey.to_lowercase();
                }
                let mut total_matches_found = 0;
                for i in 0..grind_matches_thread_safe.len() {
                    if grind_matches_thread_safe[i].count.load(Ordering::Relaxed) == 0 {
                        total_matches_found += 1;
                        continue;
                    }
                    if (!grind_matches_thread_safe[i].starts.is_empty()
                        && grind_matches_thread_safe[i].ends.is_empty()
                        && pubkey.starts_with(&grind_matches_thread_safe[i].starts))
                        || (grind_matches_thread_safe[i].starts.is_empty()
                            && !grind_matches_thread_safe[i].ends.is_empty()
                            && pubkey.ends_with(&grind_matches_thread_safe[i].ends))
                        || (!grind_matches_thread_safe[i].starts.is_empty()
                            && !grind_matches_thread_safe[i].ends.is_empty()
                            && pubkey.starts_with(&grind_matches_thread_safe[i].starts)
                            && pubkey.ends_with(&grind_matches_thread_safe[i].ends))
                    {
                        let _found = found.fetch_add(1, Ordering::Relaxed);
                        grind_matches_thread_safe[i]
                            .count
                            .fetch_sub(1, Ordering::Relaxed);
                        if !no_outfile {
                            write_keypair_file(&keypair, &format!("{}.json", keypair.pubkey()))
                                .unwrap();
                            println!("Wrote keypair to {}", &format!("{}.json", keypair.pubkey()));
                        }
                    }
                }
                if total_matches_found == grind_matches_thread_safe.len() {
                    done.store(true, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    Ok(())
}

struct GrindMatch {
    starts: String,
    ends: String,
    count: AtomicU64,
}

fn grind_parse_args(
    ignore_case: bool,
    starts_with_args: HashSet<String>,
    ends_with_args: HashSet<String>,
    num_threads: usize,
) -> Vec<GrindMatch> {
    let mut grind_matches = Vec::<GrindMatch>::new();
    for sw in starts_with_args {
        let args: Vec<&str> = sw.split(':').collect();
        if args.len() == 2 {
            grind_matches.push(GrindMatch {
                starts: if ignore_case {
                    args[0].to_lowercase()
                } else {
                    args[0].to_string()
                },
                ends: "".to_string(),
                count: AtomicU64::new(args[1].parse::<u64>().unwrap()),
            });
        } else {
            // Handle the error case where the input format is incorrect
            eprintln!("Invalid format for starts_with_args: {}", sw);
        }
    }
    for ew in ends_with_args {
        let args: Vec<&str> = ew.split(':').collect();
        if args.len() == 2 {
            grind_matches.push(GrindMatch {
                starts: "".to_string(),
                ends: if ignore_case {
                    args[0].to_lowercase()
                } else {
                    args[0].to_string()
                },
                count: AtomicU64::new(args[1].parse::<u64>().unwrap()),
            });
        } else {
            // Handle the error case where the input format is incorrect
            eprintln!("Invalid format for ends_with_args: {}", ew);
        }
    }
    // for swew in starts_and_ends_with_args {
    //     let args: Vec<&str> = swew.split(':').collect();
    //     grind_matches.push(GrindMatch {
    //         starts: if ignore_case {
    //             args[0].to_lowercase()
    //         } else {
    //             args[0].to_string()
    //         },
    //         ends: if ignore_case {
    //             args[1].to_lowercase()
    //         } else {
    //             args[1].to_string()
    //         },
    //         count: AtomicU64::new(args[2].parse::<u64>().unwrap()),
    //     });
    // }
    grind_print_info(&grind_matches, num_threads);
    grind_matches
}

pub const DEFAULT_DERIVATION_PATH: &str = "m/44'/501'/0'/0'";

pub fn derivation_path_arg<'a>() -> Arg<'a> {
    Arg::new("derivation_path")
        .long("derivation-path")
        .value_name("DERIVATION_PATH")
        .takes_value(true)
        .min_values(0)
        .max_values(1)
        .help("Derivation path. All indexes will be promoted to hardened. \
            If arg is not presented then derivation path will not be used. \
            If arg is presented with empty DERIVATION_PATH value then m/44'/501'/0'/0' will be used."
        )
}

pub fn acquire_derivation_path(
    matches: &ArgMatches,
) -> Result<Option<DerivationPath>, Box<dyn error::Error>> {
    if matches.try_contains_id("derivation_path")? {
        Ok(Some(DerivationPath::from_absolute_path_str(
            matches
                .try_get_one::<String>("derivation_path")?
                .map(|path| path.as_str())
                .unwrap_or(DEFAULT_DERIVATION_PATH),
        )?))
    } else {
        Ok(None)
    }
}

fn grind_print_info(grind_matches: &[GrindMatch], num_threads: usize) {
    println!("Searching with {num_threads} threads for:");
    for gm in grind_matches {
        let mut msg = Vec::<String>::new();
        if gm.count.load(Ordering::Relaxed) > 1 {
            msg.push("pubkeys".to_string());
            msg.push("start".to_string());
            msg.push("end".to_string());
        } else {
            msg.push("pubkey".to_string());
            msg.push("starts".to_string());
            msg.push("ends".to_string());
        }
        println!(
            "\t{} {} that {} with '{}' and {} with '{}'",
            gm.count.load(Ordering::Relaxed),
            msg[0],
            msg[1],
            gm.starts,
            msg[2],
            gm.ends
        );
    }
}

pub const WORD_COUNT_ARG: ArgConstant<'static> = ArgConstant {
    long: "word-count",
    name: "word_count",
    help: "Specify the number of words that will be present in the generated seed phrase",
};

pub struct ArgConstant<'a> {
    pub long: &'a str,
    pub name: &'a str,
    pub help: &'a str,
}

pub fn acquire_language(matches: &ArgMatches) -> Language {
    match matches
        .get_one::<String>(LANGUAGE_ARG.name)
        .unwrap()
        .as_str()
    {
        "english" => Language::English,
        "chinese-simplified" => Language::ChineseSimplified,
        "chinese-traditional" => Language::ChineseTraditional,
        "japanese" => Language::Japanese,
        "spanish" => Language::Spanish,
        "korean" => Language::Korean,
        "french" => Language::French,
        "italian" => Language::Italian,
        _ => unreachable!(),
    }
}

pub const LANGUAGE_ARG: ArgConstant<'static> = ArgConstant {
    long: "language",
    name: "language",
    help: "Specify the mnemonic language that will be present in the generated seed phrase",
};

pub fn acquire_passphrase_and_message(
    matches: &ArgMatches,
) -> Result<(String, String), Box<dyn error::Error>> {
    if matches.try_contains_id(NO_PASSPHRASE_ARG.name)? {
        Ok(no_passphrase_and_message())
    } else {
        match prompt_passphrase(
            "\nFor added security, enter a BIP39 passphrase\n\
             \nNOTE! This passphrase improves security of the recovery seed phrase NOT the\n\
             keypair file itself, which is stored as insecure plain text\n\
             \nBIP39 Passphrase (empty for none): ",
        ) {
            Ok(passphrase) => {
                println!();
                Ok((passphrase, " and your BIP39 passphrase".to_string()))
            }
            Err(e) => Err(e),
        }
    }
}

/// Prompts user for a passphrase and then asks for confirmirmation to check for mistakes
pub fn prompt_passphrase(prompt: &str) -> Result<String, Box<dyn error::Error>> {
    let passphrase = prompt_password(prompt)?;
    if !passphrase.is_empty() {
        let confirmed = rpassword::prompt_password("Enter same passphrase again: ")?;
        if confirmed != passphrase {
            return Err("Passphrases did not match".into());
        }
    }
    Ok(passphrase)
}

pub fn no_passphrase_and_message() -> (String, String) {
    (NO_PASSPHRASE.to_string(), "".to_string())
}

pub const NO_PASSPHRASE: &str = "";

pub const NO_PASSPHRASE_ARG: ArgConstant<'static> = ArgConstant {
    long: "no-bip39-passphrase",
    name: "no_passphrase",
    help: "Do not prompt for a BIP39 passphrase",
};

pub const NO_OUTFILE_ARG: ArgConstant<'static> = ArgConstant {
    long: "no-outfile",
    name: "no_outfile",
    help: "Only print a seed phrase and pubkey. Do not output a keypair file",
};
