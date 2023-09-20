fn run() -> Result<(), Box<dyn error::Error>> {
    let document = parsedHtml();
    // selectors
    let vg_sel = Selector::parse("div.vg").unwrap();
    let sc_sel = Selector::parse("div.sense-content").unwrap();
    let def_sel = Selector::parse("span.dtText").unwrap();
    let sdsense_sel = Selector::parse("div.sdsense").unwrap();
    let ex_sel = Selector::parse("span.ex-sent").unwrap();
    let sl_sel = Selector::parse("span.sl").unwrap();

    // Select first dictionary
    let vg1 = document.select(&vg_sel).next().unwrap();

    for sc in vg1.select(&sc_sel) {
        let mut definition: DefiningText = DefiningText {
            sub_label: match sc.select(&sl_sel).next() {
                Some(subject_label) => subject_label.text().collect(),
                None => String::new(),
            },
            text: sc.select(&def_sel).next().unwrap().text().collect(),
            div_sense: match sc.select(&sdsense_sel).next() {
                Some(sdsense) => sdsense.text().collect(),
                None => String::new(),
            },
            illustrations: Vec::new(),
        };
        for example in sc.select(&ex_sel) {
            let example: Illustration = Illustration {
                text: example.text().collect::<String>(),
            };
            definition.illustrations.push(example);
        }
        // add definition to word
        result.definitions.push(definition);
    }
    println!("{:?}", result);
    Ok(())
}
