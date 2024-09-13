pub struct Cutils {}

impl Cutils {
    pub fn cflags() -> Vec<&'static str> {
        let flags = "-Wjump-misses-init -Wstrict-prototypes -Wunsuffixed-float-constants -ffunction-sections -fdata-sections -Wall -Wextra -Winit-self -Wpointer-arith -Wreturn-type -Waddress -Wsequence-point -Wformat-security -Wmissing-include-dirs -Wfloat-equal -Wundef -Wshadow -Wcast-align -Wconversion -Wredundant-decls".split(" ");

        let mut cond = vec![
            "-fstack-protector-strong",
            "-DITT_fARCH_IA64",
            "-fcf-protection",
        ];
        cond.extend(flags);
        cond
    }

    pub fn enclave_cflags() -> Vec<&'static str> {
        "-ffreestanding -nostdinc -fvisibility=hidden -fpie -fno-strict-overflow -fno-delete-null-pointer-checks".split(" ").collect()
    }

    pub fn enclave_ldflags() -> Vec<&'static str> {
        "-Wl,-z,relro,-z,now,-z,noexecstack -Wl,-Bstatic -Wl,-Bsymbolic -Wl,--no-undefined -Wl,-pie -Wl,--export-dynamic -Wl,--gc-sections".split(" ").collect()
    }
}

pub(crate) fn snake_to_camel(snake: &str) -> String {
    let mut camel = String::new();
    let mut upper_next = true;

    for c in snake.chars() {
        if c == '_' {
            upper_next = true;
        } else {
            if upper_next {
                camel.push(c.to_ascii_uppercase());
                upper_next = false;
            } else {
                camel.push(c);
            }
        }
    }

    camel
}
