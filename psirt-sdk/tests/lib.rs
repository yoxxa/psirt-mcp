#[cfg(test)]
mod tests {
    use psirt_sdk::PsirtApi;

    #[test]
    fn test_psirt_api_new() {
        let psirt = PsirtApi::new();
    }
}