use crate::IconInfo;

pub struct Generator {
    pub path: &'static str,
    pub start_seq: &'static str,
    pub end_seq: &'static str,
}

impl Generator {
    pub fn generate_features<F: Fn(&IconInfo) -> String>(&self, icon_list: &[IconInfo], f: F) {
        let content = std::fs::read_to_string(self.path).unwrap();
        let mut result = String::new();
        let mut inserting = false;

        for line in content.lines() {
            if inserting {
                if line.trim_start().starts_with(self.end_seq) {
                    inserting = false;
                } else {
                    continue;
                }
            }

            result.push_str(line);
            result.push('\n');

            if line.trim_start().starts_with(self.start_seq) {
                inserting = true;

                for icon in icon_list {
                    result.push_str(&f(icon));
                    result.push('\n');
                }
            }
        }

        std::fs::write(self.path, result).unwrap();
    }
}
