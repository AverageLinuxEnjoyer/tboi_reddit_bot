pub fn extract_strings_between(input: &str) -> Vec<String> {
    let mut res = vec![];
    let mut curr = String::new();

    let mut it = input.chars().peekable();

    let mut inside = false;

    while let Some(ch) = it.next() {
        let next_ch = match it.peek() {
            Some(next_ch) => *next_ch,
            None => break,
        };

        match ch {
            '{' if next_ch == '{' => {
                inside = true;
                it.next();
            }
            '}' if next_ch == '}' && inside => {
                inside = false;
                it.next();
            }
            ch => match inside {
                true => {
                    curr.push(ch);
                }
                false => {
                    if !curr.is_empty() {
                        res.push(curr.clone());
                        curr.clear();
                    }
                }
            },
        };
    }

    if !inside && !curr.is_empty() {
        res.push(curr);
    }

    res
}
