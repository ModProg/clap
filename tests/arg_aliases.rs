mod utils;

use clap::{App, Arg};

static SC_VISIBLE_ALIAS_HELP: &str = "ct-test 1.2

Some help

USAGE:
    ct test [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       [aliases: v_flg, flag2, flg3]
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -o, --opt <opt>    [aliases: visible]";

static SC_INVISIBLE_ALIAS_HELP: &str = "ct-test 1.2

Some help

USAGE:
    ct test [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -o, --opt <opt>    ";

#[test]
fn single_alias_of_option() {
    let a = App::new("single_alias")
        .arg(
            Arg::new("alias")
                .long("alias")
                .takes_value(true)
                .about("single alias")
                .alias("new-opt"),
        )
        .try_get_matches_from(vec!["", "--new-opt", "cool"]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("alias"));
    assert_eq!(a.value_of("alias").unwrap(), "cool");
}

#[test]
fn multiple_aliases_of_option() {
    let a = App::new("multiple_aliases").arg(
        Arg::new("aliases")
            .long("aliases")
            .takes_value(true)
            .about("multiple aliases")
            .aliases(&["alias1", "alias2", "alias3"]),
    );
    let long = a
        .clone()
        .try_get_matches_from(vec!["", "--aliases", "value"]);
    assert!(long.is_ok());
    let long = long.unwrap();

    let als1 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias1", "value"]);
    assert!(als1.is_ok());
    let als1 = als1.unwrap();

    let als2 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias2", "value"]);
    assert!(als2.is_ok());
    let als2 = als2.unwrap();

    let als3 = a
        .clone()
        .try_get_matches_from(vec!["", "--alias3", "value"]);
    assert!(als3.is_ok());
    let als3 = als3.unwrap();

    assert!(long.is_present("aliases"));
    assert!(als1.is_present("aliases"));
    assert!(als2.is_present("aliases"));
    assert!(als3.is_present("aliases"));
    assert_eq!(long.value_of("aliases").unwrap(), "value");
    assert_eq!(als1.value_of("aliases").unwrap(), "value");
    assert_eq!(als2.value_of("aliases").unwrap(), "value");
    assert_eq!(als3.value_of("aliases").unwrap(), "value");
}

#[test]
fn single_alias_of_flag() {
    let a = App::new("test")
        .arg(Arg::new("flag").long("flag").alias("alias"))
        .try_get_matches_from(vec!["", "--alias"]);
    assert!(a.is_ok());
    let a = a.unwrap();
    assert!(a.is_present("flag"));
}

#[test]
fn multiple_aliases_of_flag() {
    let a = App::new("test").arg(Arg::new("flag").long("flag").aliases(&[
        "invisible",
        "set",
        "of",
        "cool",
        "aliases",
    ]));

    let flag = a.clone().try_get_matches_from(vec!["", "--flag"]);
    assert!(flag.is_ok());
    let flag = flag.unwrap();

    let inv = a.clone().try_get_matches_from(vec!["", "--invisible"]);
    assert!(inv.is_ok());
    let inv = inv.unwrap();

    let cool = a.clone().try_get_matches_from(vec!["", "--cool"]);
    assert!(cool.is_ok());
    let cool = cool.unwrap();

    let als = a.clone().try_get_matches_from(vec!["", "--aliases"]);
    assert!(als.is_ok());
    let als = als.unwrap();

    assert!(flag.is_present("flag"));
    assert!(inv.is_present("flag"));
    assert!(cool.is_present("flag"));
    assert!(als.is_present("flag"));
}

#[test]
fn alias_on_a_subcommand_option() {
    let m = App::new("test")
        .subcommand(
            App::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .alias("opt")
                    .about("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other").aliases(&["o1", "o2", "o3"]))
        .get_matches_from(vec!["test", "some", "--opt", "awesome"]);

    assert!(m.subcommand_matches("some").is_some());
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "awesome");
}

#[test]
fn invisible_arg_aliases_help_output() {
    let app = App::new("ct").author("Salim Afiune").subcommand(
        App::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .aliases(&["invisible", "als1", "more"]),
            )
            .arg(Arg::from("-f, --flag").aliases(&["unseeable", "flg1", "anyway"])),
    );
    assert!(utils::compare_output(
        app,
        "ct test --help",
        SC_INVISIBLE_ALIAS_HELP,
        false
    ));
}

#[test]
fn visible_arg_aliases_help_output() {
    let app = App::new("ct").author("Salim Afiune").subcommand(
        App::new("test")
            .about("Some help")
            .version("1.2")
            .arg(
                Arg::new("opt")
                    .long("opt")
                    .short('o')
                    .takes_value(true)
                    .alias("invisible")
                    .visible_alias("visible"),
            )
            .arg(
                Arg::new("flg")
                    .long("flag")
                    .short('f')
                    .visible_aliases(&["v_flg", "flag2", "flg3"]),
            ),
    );
    assert!(utils::compare_output(
        app,
        "ct test --help",
        SC_VISIBLE_ALIAS_HELP,
        false
    ));
}
