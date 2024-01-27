use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{IsTerminal, Read, stdin};
use std::path::{Path, PathBuf};

fn main() {
    let mut ext_op = "".to_string();
    let mut final_paths :Vec<String> = Vec::new();
    let mut file_paths :Vec<String> = env::args().skip(1).collect();

    if file_paths.is_empty() {
        let mut s = String::new();
        let mut sin = stdin();
        if sin.is_terminal() {
            println!("use stdin or rtc [file_extension or '*'] [file_path]");
            return;
        }
        sin.read_to_string(&mut s).expect("");
        file_paths = s.split("\r").map(|x| x.to_string()).collect();
    } else {
        ext_op = file_paths.get(0).unwrap().clone();
        file_paths = file_paths.iter().map(|x| x.clone()).skip(1).collect();
    }

    file_paths.sort();

    for file_path in file_paths {
        let p = Path::new(&file_path);
        if p.is_dir() {
            let mut ses = read_dir(&file_path);
            if ext_op != "*" {
                ses = ses.iter()
                    .filter(|&x| x.ends_with(ext_op.as_str()))
                    .map(|x| x.clone())
                    .collect();
            }

            final_paths.append(&mut ses);
        } else {
            final_paths.push(p.display().to_string());
        }
    }

    let data_map = files_counter(final_paths);
    display(data_map);
}

fn files_counter(files :Vec<String>) -> HashMap<String, (usize, usize)> {
    let mut map = HashMap::new();

    for file_string in files {
        let mut file = File::open(&file_string).expect("no file");
        let mut s = String::new();
        let res = file.read_to_string(&mut s);
        let x = char_count(&mut s);
        match res {
            Ok(_) => {
                map.insert(file_string, x);
            }
            Err(_) => continue,
        }
    }

    map
}

fn display(data :HashMap<String, (usize, usize)>) {
    let counts :Vec<(usize, usize)> = data.values()
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect();
    let total = counts.iter()
        .fold((0, 0), |(c, l), &(x, y)| (c + x, l +y));
    let count = counts.iter().map(|x| (x.0.to_string().len(), x.1.to_string().len()))
        .fold((0,0), |(c, l), (x, y)| {
        (
            if c < x {x} else {c},
            if l < y {y} else {l}
        )
    });
    let names :Vec<usize> = data.keys().map(|x| x.len()).collect();
    let name_max = names.iter().fold(0, |c, &x | if c < x {x} else {c});

    let name_sp = " ".repeat(over(name_max));
    let char_sp = " ".repeat(over(count.0));
    let line_sp = " ".repeat(over(count.1));
    let lc_sp = char_sp.clone() + line_sp.as_str();

    print!("{}Name{}    Char{}{}Line\n",name_sp.clone() + if name_sp.len() < 7 {" "} else {""}, name_sp, char_sp, line_sp );
    print!("{}    {}{}{}\n", "-".repeat(u_under(name_max)),
           "-". repeat(u_under(count.0)),
           lc_sp,
           "-".repeat(u_under(count.1)));

    for (k, v) in data {
        print!("{}    {}{}{}{}\n", k, " ".repeat(over_s(name_max, k.len())), v.0,
               format!("{}{}"," ".repeat(over_s(count.0, v.0.to_string().len())),lc_sp), v.1);
    }

    print!("\n");
    print!("total(Char) {}\n", total.0);
    print!("total(Line) {}\n", total.1);
}

fn u_under(u :usize) -> usize {
    if u < 4 {
        4
    } else {
        u
    }
}

fn over(u :usize) -> usize {
    if u <=4 {
        1
    } else {
        let i = u as i64 -4;
        let i = i /2;
        if i <= 4 {
            1
        } else {
            let f = (i as f64).floor();
            f as usize
        }
    }
}

fn over_s(u :usize, i :usize) -> usize {
    let mut f = (u as i64) - (i as i64);

    if u < 4 {
        f = 4 - (i as i64);
    }

    if f < 0 {
        0
    } else {
        f as usize
    }
}

fn read_dir(p :&String) -> Vec<String> {
    let mut results :Vec<String> = Vec::new();
    let dir = PathBuf::from(p);
    let files = dir.read_dir().unwrap();
    for entry in files {
        let file_path = entry.unwrap().path();
        let path_string = file_path.to_str().unwrap().to_string();
        if file_path.is_dir() {
            let s = read_dir(&path_string);
            s.iter().for_each(|x| results.push(x.to_string()));
        } else {
            results.push(path_string);
        }
    }

    results
}

fn char_count(s: &mut String) -> (usize, usize) {
    (s.replace("\n","")
        .replace("\r", "")
        .chars()
        .count(),
    s.split("\n").count()
    )
}
