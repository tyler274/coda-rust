pub const WINDOW: usize = 400;
pub const STRIDE: usize = 352;
pub const BATCH: usize = 16;

/// Content-token windows: `window` tokens, stepped by `stride`, with a short final window.
/// An empty token list has nothing to represent.
pub fn content_windows(ids: &[u32], window: usize, stride: usize) -> Vec<Vec<u32>> {
    if ids.is_empty() {
        return Vec::new();
    }
    let mut windows = Vec::new();
    let mut start = 0;
    loop {
        let end = (start + window).min(ids.len());
        windows.push(ids[start..end].to_vec());
        if start + window >= ids.len() {
            break;
        }
        start += stride;
    }
    windows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counts(tokens: usize) -> Vec<usize> {
        let ids = vec![1; tokens];
        content_windows(&ids, WINDOW, STRIDE)
            .into_iter()
            .map(|window| window.len())
            .collect()
    }

    #[test]
    fn window_counts_match_the_documented_edges() {
        assert_eq!(counts(0), Vec::<usize>::new());
        assert_eq!(counts(1), vec![1]);
        assert_eq!(counts(399), vec![399]);
        assert_eq!(counts(400), vec![400]);
        assert_eq!(counts(401).len(), 2);
        assert_eq!(counts(752).len(), 2);
        assert_eq!(counts(753).len(), 3);
        assert_eq!(counts(1_104).len(), 3);
        assert_eq!(counts(1_105).len(), 4);
        assert_eq!(counts(5_990).len(), 17);
        assert_eq!(counts(401)[0], 400);
        assert_eq!(counts(401)[1], 49);
    }
}
