pub fn get_message_chunks(message: &str, limit: usize) -> Vec<&str> {
    if message.len() < limit {
        vec![message]
    } else {
        let mut offset = 0_usize;
        let mut previous = 0_usize;

        let mut indexes: Vec<(usize, usize)> = Vec::new();

        for (idx, char) in message.char_indices() {
            if char == '\n' {
                if idx - previous + 1 > limit {
                    indexes.push((previous, offset));
                    previous = offset;
                }
                offset = idx + 1;
            }
        }

        indexes.push((previous, message.len()));

        indexes
            .iter()
            .map(|(start, end)| &message[*start..*end])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::alerts::utils::get_message_chunks;

    #[tokio::test]
    async fn check_slack_message_split() {
        let message = "q\nwr\nert\nfg\nzs";
        let blocks = get_message_chunks(message, 5);

        assert_eq!(
            *blocks,
            ["q\nwr\n", "ert\n", "fg\nzs"]
        )
    }
}
