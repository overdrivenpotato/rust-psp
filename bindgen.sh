#!/usr/bin/env bash

# Quick and dirty script to generate psp_extern! bindings from a PSPSDK header.

header() {
    bindgen $1 --no-layout-tests --whitelist-function 'sce.*' -- \
        -I /usr/local/pspdev/lib/gcc/psp/9.3.0/include \
        -I /usr/local/pspdev/psp/sdk/include \
        -I /usr/local/pspdev/psp/include \
        -include sys/types.h -include pspkerneltypes.h \
        | sed -re '
            # Format doc comments as triple slashes
            s/#\[doc = "([^"]*)"\]/\/\/\/\1/g

            # Remove redundant raw types
            s/::std::os::raw::c_int/i32/g
            s/::std::os::raw::c_uint/u32/g
            s/::std::os::raw::c_void/c_void/g
            s/::std::os::raw::c_char/u8/g
            s/::std::os::raw::c_uchar/u8/g
            s/::std::os::raw::c_ulong/u32/g
            s/::std::os::raw::c_long/i32/g

            s/uint/u32/g

            s/SceSize/usize/g
            s/SceSSize/isize/g
            s/SceVoid/c_void/g

            s/SceInt32/i32/g
            s/SceShort16/i16/g

            s/SceUInt32/u32/g
            s/SceUChar8/u8/g

            s/@param (\w*)\s+- /- `\1`: /
            s/@return (.*)/# Return Value\n    \/\/\/\n    \/\/\/ \1/

            # Delete redundant types resulting from previous replacements
            /pub type (\w+) = (\1);/d
            /pub type (usize|isize) =/d
        ' \
        | awk '
            # Bindgen generates multiple extern blocks so we want to merge them.
            /^}$/ { if (found_extern) { printf "\n"; next } }
            /^extern/ {
                if (!found_extern) {
                    found_extern = 1;
                    printf "\npsp_extern! {\n";
                }

                next;
            }

            # Add parameters header
            /\/\/\/ - `/ { if (!params) printf "    /// # Parameters\n    ///\n" }
            { if ($0 ~ /\s*\/\/\/ - `/) params = 1; else params = 0 }

            { print }

            END { print "}" }
        ' \
        | rustfmt
}

pspModule() {
    grep 'IMPORT_' < $1 \
        | sed '
            s/.*IMPORT_START\s*//
            s/.*IMPORT_FUNC.*",//
        ' \
        | cat - <(echo) # Add newline
}

# Utility function for awk
AWK_CAMEL='
    function camelToSnake(s,    last, i) {
        out = ""

        for (i = 0; i < length(s); i++) {
            char = substr(s, i + 1, 1)

            if (tolower(char) != char) {
                if (last != i - 1) out = out "_"
                last = i
                out = out tolower(char)
            } else {
                out = out char
            }
        }

        return out
    }
'

cat <(pspModule $2) <(header $1) \
    | awk "$AWK_CAMEL"'
        /^$/ { header_done = 1 }

        {
            if (!name) {
                name = substr($0, 1, match($0, ",") - 1)
                flags = substr($0, match($0, ",") + 1 + 2, 4)
                maj_ver = substr($0, match($0, ",") + 1 + 6, 2)
                min_ver = substr($0, match($0, ",") + 1 + 8, 2)

                next
            } else if (!header_done) {
                nid_name = substr($0, match($0, ",") + 1)
                nid_hex = substr($0, 1, match($0, ",") - 1)

                # Remove carriage returns
                gsub("\r", "", nid_name)

                nid_map[nid_name] = nid_hex

                next
            }
        }

        # Fix camel case struct fields
        /\s*pub \w*:/ {
            # field name start and end index
            start = match($0, "pub ") + 4
            end = match($0, ":")
            camel = substr($0, start, end - start)

            printf "%s%s%s\n", substr($0, 1, start - 1), camelToSnake(camel), substr($0, end)

            next
        }

        /psp_extern!/ {
            print
            print "    #![name = " name "]"
            print "    #![flags = 0x" flags "]"
            print "    #![version = (0x" maj_ver  ", 0x" min_ver ")]"
            printf "\n"

            RS = "\n\n"

            next
        }

        /pub fn/ {
            # Function name start and end index
            fn_start = match($0, "fn ") + 3
            fn_end = match(substr($0, fn_start), "\\(") + fn_start
            fn_camel = substr($0, fn_start, fn_end - fn_start - 1)

            printf "    #[psp(" nid_map[fn_camel] ")]\n"
            printf substr($0, 1, fn_start - 1)
            printf camelToSnake(fn_camel) "("

            # Substring of the argument list
            args = substr($0, fn_end, match(substr($0, fn_end), ")") - 1)

            args_multiline = match(args, "\n")
            fn_return = fn_end + length(args)

            if (args_multiline) printf "\n"

            # Fix the argument names
            while (1) {
                # These are indices
                arg_start = match(args, "[^[:space:]]")
                arg_colon = match(args, ":")
                arg_comma = match(args, ",")

                # Exit if no more arguments
                if (!arg_start) break

                arg_len = arg_comma - arg_start

                # Get the camelCase and snake_case variants of the argument name
                arg_camel = substr(args, arg_start, arg_colon - arg_start)
                arg_snake = camelToSnake(arg_camel)

                # Recreate the argument definition with the correct case
                if (arg_comma) arg = substr(args, arg_start, arg_len)
                else           arg = substr(args, arg_start)
                sub(arg_camel, arg_snake, arg)

                if (args_multiline) printf "        " arg ",\n"
                else if (arg_comma) printf arg ", "
                else                printf arg

                # Advance the argument substring onto the next one
                if (arg_comma) args = substr(args, arg_comma + 1)
                else           break
            }

            if (args_multiline) printf "    "

            print substr($0, fn_return)
            printf "\n"

            next
        }

        { print }
    ' \
    | awk "$AWK_CAMEL"'
        # Fix camel case doc comment parameter names
        /\s*\/\/\/ - `/ {
            start = index($0, "`") + 1
            end = index(substr($0, start), "`") + start
            camel = substr($0, start, end - start)

            sub(camel, camelToSnake(camel), $0)

            print $0
            next
        }

        { print }
    '
