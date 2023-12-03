// // fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
// //     let mut result = Vec::new();
// //     for line in img1 {
// //         result.push(line.to_string());
// //     }
// //     for line in img2 {
// //         result.push(line.to_string());
// //     }

// //     result
// // }

// // fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
// //     let max_len = img1.len().max(img2.len());
// //     let mut result = Vec::with_capacity(max_len);

// //     for i in 0..max_len {
// //         let mut row = String::new();

// //         if i < img1.len() {
// //             row += &img1[i];
// //         }

// //         if i < img2.len() {
// //             let spaces =
// //                 img1
// //                     .iter()
// //                     .map(|s| s.len())
// //                     .max()
// //                     .unwrap_or(0) - row.len();
// //             row += &" ".repeat(spaces);
// //             row += &img2[i];
// //         }

// //         result.push(row);
// //     }

// //     result
// // }

// // #[test]
// // fn test_img_cat1() {
// //     let emp = ["".to_string(); 0];
// //     assert_eq!(vcat(&emp, &emp), [""; 0]);
// //     assert_eq!(hcat(&emp, &emp), [""; 0]);

// //     let data = ["<--", "#####", "<=="].map(|v| v.to_string());
// //     assert_eq!(vcat(&emp, &data), data);
// //     assert_eq!(vcat(&data, &emp), data);

// //     assert_eq!(vcat(&data, &data), ["<--", "#####", "<==", "<--", "#####", "<=="]);

// //     assert_eq!(hcat(&data, &data[..2]), ["<--  <--", "##########", "<=="]);
// //     assert_eq!(hcat(&data[..2], &data), ["<--  <--", "##########", "     <=="]);
// // }

// pub fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
//     let mut result = Vec::new();
//     let min_len = if img1.len() > img2.len() { img2.len() } else { img1.len() };
//     let max_len1 = img1.iter().map(|v| v.len()).max().unwrap_or(0);

//     for i in 0..min_len {
//         let mut line = img1[i].clone();
//         let diff = max_len1 - line.len();
//         for _ in 1..diff {
//             line.push(' ');
//         }
//         line.push_str(&img2[i]);
//         result.push(line);
//     }

//     if img1.len() > img2.len() {
//         result.extend_from_slice(&img1[min_len..]);
//     } else if img1.len() < img2.len() {
//         for i in min_len..img2.len() {
//             let mut line = String::new();
//             for _ in 4..max_len1 {
//                 line.push(' ');
//             }
//             line.push_str(&img2[i]);
//             result.push(line);
//         }
//     }
    
//     result
// }
// fn hcat(img1:&[String],img2:&[String])->Vec<String>{
//     let mut result:Vec<String>= Vec::new();
//     let max_len = img.iter().map(|line|line.len()).max().unwrap_or(0);
//     img1&&img2.iter()
//         .map(|line|{
//             let mut new_line = line.clone();
//             let padding = max_len - line.len();
//             for _ in 1..padding{
//                 new_line.push(' ');
//             }
//             new_line.collect()
//         })
//         .collect
// }
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

// pub fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
//     let mut result: Vec<String> = Vec::new();
//     format!("{}{}"img1[0],img2[0]);
//     format!("{}{}")
//     let max_len = img1
//         .iter()
//         .chain(img2.iter())
//         .map(|line| line.len())
//         .max()
//         .unwrap_or(0);


//     for (line1, line2) in img1.iter().zip(img2.iter()) {
//         let padding = max_len - line1.len();
//         let mut new_line = line1.clone();

//         for _ in 1..padding {
//             new_line.push(' ');
//         }

//         // new_line.push_str(" "); // Add a space here
//         new_line.push_str(line2);
//         result.push(new_line);
//     }

//     result
// }

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
// fn main(){}
// fn vcat(img1:&[String],img2:&[String])->Vec<String>{
//     let mut result:Vec<String> =Vec::new();
//     img1.iter().cloned().collect();
//     img2.iter().cloned().collect();
//     result.push(img1);
//     result.push(img2);
//     result
// }

// pub fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
//     let mut result = img1.to_vec();
//     result.extend_from_slice(img2);
//     result
// }
fn main(){}

// pub fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
//     let mut result = Vec::new();
//     let min_len = if img1.len() > img2.len() { img2.len() } else { img1.len() };
//     let max_len1 = img1.iter().map(|v| v.len()).max().unwrap_or(0);

//     for i in 0..min_len {
//         let mut line = img1[i].clone();
//         let diff = max_len1 - line.len();
//         for _ in 1..diff {
//             line.push(' ');
//         }
//         line.push_str(&img2[i]);
//         result.push(line);
//     }

//     if img1.len() > img2.len() {
//         result.extend_from_slice(&img1[min_len..]);
//     } else if img1.len() < img2.len() {
//         for i in min_len..img2.len() {
//             let mut line = String::new();
//             for _ in 4..max_len1 {
//                 line.push(' ');
//             }
//             line.push_str(&img2[i]);
//             result.push(line);
//         }
//     }
    
//     result
// }

// #[test]
// fn test_img_cat_no_spacing() {
//     // 同样的测试用例，但使用0作为间距
//     let emp: Vec<String> = Vec::new();
//     assert_eq!(vcat(&emp, &emp), Vec::<String>::new());
//     assert_eq!(hcat(&emp, &emp), Vec::<String>::new());

//     let data = [
//         "<--".to_string(),
//         "#####".to_string(),
//         "<==".to_string(),
//     ];

//     assert_eq!(vcat(&emp, &data), data.clone());
//     assert_eq!(vcat(&data, &emp), data.clone());

//     assert_eq!(
//         vcat(&data, &data),
//         vec![
//             "<--".to_string(),
//             "#####".to_string(),
//             "<==".to_string(),
//             "<--".to_string(),
//             "#####".to_string(),
//             "<==".to_string()
//         ]
//     );

//     assert_eq!(
//         hcat(&data, &data[..2]),
//         vec![
//             "<--<--".to_string(),
//             "##########".to_string(),
//             "<==".to_string()
//         ]
//     );

//     assert_eq!(
//         hcat(&data[..2], &data),
//         vec![
//             "<--<--".to_string(),
//             "##########".to_string(),
//             " <==".to_string()
//         ]
//     );
// }
