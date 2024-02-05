pub mod markdown_conv {
    const MAX_HEADING_SIZE: usize = 6;
    fn convert_heading(markdown: String) -> Option<String> {
        let whitespace_pos = markdown.find(' ');
        match whitespace_pos{
            Some(pos) =>{
                let (size, text) = markdown.split_at(pos);
                let size = if size.len() <= MAX_HEADING_SIZE { size.len()} else{ return None};
                let text = text.strip_prefix(' ');

                text.map(|t| format!("<h{size}>{t}</h{size}>"))

            },
            None => None,
        }
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_heading_convertion(){
            let markdown = String::from("# Test");
            assert_eq!(convert_heading(markdown), Some(String::from("<h1>Test</h1>")));

            let markdown = String::from("###### Test");
            assert_eq!(convert_heading(markdown), Some(String::from("<h6>Test</h6>")));

            let markdown = String::from("######Test");
            assert_eq!(convert_heading(markdown), None);

        }
    }
}