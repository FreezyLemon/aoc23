pub struct Day15Part2;

impl crate::days::Day for Day15Part2 {
    fn solve(&self, input: &str) -> String {
        let mut boxes = vec![Vec::<&str>::new(); 256];

        input.split(',')
            .for_each(|step| {
                if let Some(i) = step.find('=') {
                    let label = &step[..i];
                    let idx = create_hash(label) as usize;
                    let inner_box = &mut boxes[idx];
                    
                    if let Some((old_entry_idx, _)) = inner_box.iter()
                        .enumerate()
                        .find(|(_, entry)| entry.starts_with(label))
                    {
                        inner_box[old_entry_idx] = step;
                    }
                    else
                    {
                        inner_box.push(step);
                    }
                } else if let Some(i) = step.find('-') {
                    let label = &step[..i];
                    let idx = create_hash(label) as usize;
                    let inner_box = &mut boxes[idx];

                    if let Some((old_entry_idx, _)) = inner_box.iter()
                        .enumerate()
                        .find(|(_, entry)| entry.starts_with(label))
                    {
                        inner_box.remove(old_entry_idx);
                    }
                } else {
                    panic!("step should contain = or -")
                };
            });
        
        boxes.into_iter()
            .enumerate()
            .flat_map(|(box_no, inner_box)|
                inner_box.into_iter()
                    .map(|lens| lens.chars().last().unwrap().to_digit(10).unwrap())
                    .enumerate()
                    .map(move |(lens_slot, focal_len)| (1 + box_no as u32) * (1 + lens_slot as u32) * focal_len)
            )
            .sum::<u32>()
            .to_string()
    }
}

fn create_hash(s: &str) -> u8 {
    let mut res: u16 = 0;

    for b in s.bytes() {
        res += u16::from(b);
        res *= 17;
        res %= 256;
    }

    u8::try_from(res).expect("value fits into u8")
}
