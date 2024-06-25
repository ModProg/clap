complete -c my-app -n "__fish_use_subcommand" -s c -s C -l config -l conf -d 'some config file'
complete -c my-app -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my-app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'top level subcommand'
complete -c my-app -n "__fish_use_subcommand" -f -a "some_cmd_alias" -d 'top level subcommand'
complete -c my-app -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c my-app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -l config -d 'the other case to test' -r -f -a "{Lest quotes\, aren\'t escaped.\t'help,with,comma',Second to trigger display of options\t''}"
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -l config -d 'the other case to test' -r -f -a "{Lest quotes\, aren\'t escaped.\t'help,with,comma',Second to trigger display of options\t''}"
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and __fish_seen_subcommand_from sub_cmd" -s V -l version -d 'Print version'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my-app -n "__fish_seen_subcommand_from some_cmd_alias; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from help" -f -a "test" -d 'tests things'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from help" -f -a "some_cmd" -d 'top level subcommand'
complete -c my-app -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from test; and not __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my-app -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd" -f -a "sub_cmd" -d 'sub-subcommand'
