pub fn vflip(img: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in img.iter().rev() {
        result.push(line.to_string());
    }

    result
}


fn hflip(img: &[String]) -> Vec<String> {
    let max_len = img
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);
    let mut result = Vec::new();

    for line in img {
        let reversed_line = format!(
            "{:>width$}",
            line.chars().rev().collect::<String>(),
            width = max_len
        );
        result.push(reversed_line);
    }

    result
}

#[test]
fn test_img_flip() {
    let emp: Vec<String> = Vec::new();
    assert_eq!(vflip(&emp), Vec::<String>::new());
    assert_eq!(hflip(&emp), Vec::<String>::new());

    let data = [
        "<--".to_string(),
        "#####".to_string(),
        "<==".to_string(),
    ];

    assert_eq!(
        vflip(&data),
        vec![
            "<==".to_string(),
            "#####".to_string(),
            "<--".to_string()
        ]
    );

    assert_eq!(
        hflip(&data),
        vec![
            " --<".to_string(),
            "#####".to_string(),
            " ==<".to_string()
        ]
    );
}
fn main(){}
