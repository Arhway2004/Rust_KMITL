fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in img1 {
        result.push(line.to_string());
    }
    for line in img2 {
        result.push(line.to_string());
    }

    result
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let max_len = img1.len().max(img2.len());
    let mut result = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let mut row = String::new();

        if i < img1.len() {
            row += &img1[i];
        }

        if i < img2.len() {
            let spaces =
                img1
                    .iter()
                    .map(|s| s.len())
                    .max()
                    .unwrap_or(0) - row.len();
            row += &" ".repeat(spaces);
            row += &img2[i];
        }

        result.push(row);
    }

    result
}


#[test]
fn test_img_cat() {
    let emp: Vec<String> = Vec::new();
    assert_eq!(vcat(&emp, &emp), Vec::<String>::new());
    assert_eq!(hcat(&emp, &emp), Vec::<String>::new());

    let data = [
        "<--".to_string(),
        "#####".to_string(),
        "<==".to_string(),
    ];

    assert_eq!(vcat(&emp, &data), data.clone());
    assert_eq!(vcat(&data, &emp), data.clone());

    assert_eq!(
        vcat(&data, &data),
        vec![
            "<--".to_string(),
            "#####".to_string(),
            "<==".to_string(),
            "<--".to_string(),
            "#####".to_string(),
            "<==".to_string()
        ]
    );

    assert_eq!(
        hcat(&data, &data[..2]),
        vec![
            "<--  <--".to_string(),
            "##########".to_string(),
            "<==".to_string()
        ]
    );

    assert_eq!(
        hcat(&data[..2], &data),
        vec![
            "<--  <--".to_string(),
            "##########".to_string(),
            "     <==".to_string()
        ]
    );
}
fn main(){}