#!/usr/bin/env bash

# Quick and dirty script to generate sys_lib! bindings from a PSPSDK header.

header() {
    bindgen $1 --no-layout-tests --whitelist-function 'sce.*' -- \
        -I /usr/local/pspdev/lib/gcc/psp/9.3.0/include \
        -I /usr/local/pspdev/psp/sdk/include \
        -I /usr/local/pspdev/psp/include \
        -include sys/types.h -include pspkerneltypes.h \
        | sed -re '
            s/#\[doc = "([^"]*)"\]/\/\/\/\1/g
            s/::std::os::raw::c_int/i32/g
            s/::std::os::raw::c_uint/u32/g
            s/::std::os::raw::c_void/c_void/g
            s/pub fn/pub unsafe fn/
            s/@param (\w*)/`\1`/
            s/@return (.*)/# Return Value\n    \/\/\/\n    \/\/\/ \1/
        ' \
        | awk '
            # Bindgen generates multiple extern blocks so we want to merge them.
            /^}$/ { if (found_extern) next }
            /^extern/ {
                if (!found_extern) {
                    found_extern = 1;
                    printf "\nsys_lib! {\n";
                }

                next;
            }

            # Add parameters header
            /\/\/\/ `/ { if (!params) printf "    /// # Parameters\n    ///\n" }
            { if ($0 ~ /\s*\/\/\/ `/) params = 1; else params = 0 }

            # TODO: Needed?
            { print }

            /-> \w+;/ { printf "\n"; }

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

cat <(pspModule $2) <(header $1 | rustfmt) \
    | awk '
        /^$/ { header_done = 1 }

        {
            if (!name) {
                name = substr($0, 1, match($0, ",") - 1)
                flags = substr($0, match($0, ",") + 1 + 2, 4)
                maj_ver = substr($0, match($0, ",") + 1 + 6, 2)
                min_ver = substr($0, match($0, ",") + 1 + 8, 2)

                next
            } else if (!header_done) {
                nid_map[substr($0, match($0, ",") + 1)] = substr($0, 1, match($0, ",") - 1)
                next
            }
        }

        function camelToSnake(s,    last) {
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

        # Camel case struct fields
        /\s*pub \w*:/ {
            # field name start and end index
            start = match($0, "pub ") + 4
            end = match($0, ":")
            camel = substr($0, start, end - start)

            printf "%s%s%s\n", substr($0, 1, start - 1), camelToSnake(camel), substr($0, end)

            next
        }

        /sys_lib!/ {
            print
            print "    #![name = " name "]"
            print "    #![flags = 0x" flags "]"
            print "    #![version = (0x" maj_ver  ", 0x" min_ver ")]"
            printf "\n"

            RS = "\n\n"

            next
        }

        /pub unsafe fn/ {
            # Function name start and end index
            start = match($0, "fn ") + 3
            end = match(substr($0, start), "\\(") + start
            camel = substr($0, start, end - start - 1)

            printf "    #[psp(" nid_map[camel] ")]\n"
            printf substr($0, 1, start - 1)
            printf camelToSnake(camel)
            print substr($0, end - 1)
            printf "\n"

            next
        }

        { print }
    '
