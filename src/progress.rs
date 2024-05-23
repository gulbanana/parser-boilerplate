pub type Command = (&'static str, &'static str);

pub struct Quest {
    command: Command,
    item: String,
    superuser: bool,
}

pub const COMMANDS: &'static [Command] = &[("make", "made"), ("deliver", "delivered")];

pub fn interpret(quests: &[Quest]) -> String {
    let mut result = String::new();
    for quest in quests.iter() {
        result.push_str(quest.item.as_str());
        result.push(' ');
        result.push_str(quest.command.1);
        if quest.superuser {
            result.push_str(" as admin");
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{interpret, Quest};

    #[test]
    fn interpret_single() {
        assert_eq!(
            "egg made\n",
            interpret(&[Quest {
                command: ("make", "made"),
                item: "egg".into(),
                superuser: false,
            }])
        );
    }

    #[test]
    fn interpret_multiple() {
        assert_eq!(
            "bed made\nbed made as admin\n",
            interpret(&[
                Quest {
                    command: ("make", "made"),
                    item: "bed".into(),
                    superuser: false,
                },
                Quest {
                    command: ("make", "made"),
                    item: "bed".into(),
                    superuser: true,
                }
            ])
        );
    }
}
