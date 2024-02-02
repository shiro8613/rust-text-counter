use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{IsTerminal, Read, stdin};
use std::path::{Path, PathBuf};

const LEN_PLUS: i64 = 2;

fn main() {
    let mut ext_op = "".to_string();
    let mut final_paths :Vec<String> = Vec::new();
    let mut file_paths :Vec<String> = env::args().skip(1).collect();

    if file_paths.is_empty() {
        let mut s = String::new();
        let mut sin = stdin();
        if sin.is_terminal() {
            println!("use stdin or rwc [file_extension or '%'] [file_path]");
            return;
        }
        sin.read_to_string(&mut s).expect("");
        file_paths = s.split("\r")
            .map(|x| x.to_string())
            .map(|x| x.replace("\n", ""))
            .map(|x| x.replace("\r", ""))
            .filter(|x| !x.is_empty())
            .collect();
    } else {
        if let Some(ext) = file_paths.get(0) {
            ext_op = ext.clone();
        }

        file_paths = file_paths.iter().map(|x| x.clone()).skip(1).collect();
    }

    file_paths.sort();

    for file_path in file_paths {
        let p = Path::new(&file_path);
        if p.is_dir() {
            let mut ses = read_dir(&file_path);
            if ext_op != "%" {
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
        let file_res = File::open(&file_string);
        if let Ok(mut file) = file_res {
            let mut s = String::new();
            let res = file.read_to_string(&mut s);
            let x = char_count(&mut s);
            if let Ok(_) = res {
                map.insert(file_string, x);
            }
        }
    }

    map
}

fn display(data :HashMap<String, (usize, usize)>) {
    let count_total = data.values().fold((0, 0), |(x,y), &(x1, y1)| (x + (x1 as i64), y + (y1 as i64)));

    let mut data_vec :Vec<(String, String, String)> = data.iter()
        .map(|(k,v)| (k.clone(), v.0.clone(), v.1.clone()))
        .map(|(k,x,y)| (k, x.to_string(), y.to_string()))
        .collect();

    data_vec.insert(0, ("Name".to_string(), "Char".to_string(), "Line".to_string()));
    data_vec.push(("Total".to_string(), count_total.0.to_string(), count_total.1.to_string()));

    let data_all_len = data_vec.iter()
        .map(|(x,y,z)| (x.len(), y.len(), z.len()))
        .map(|(x, y, z)| (x as i64, y as i64, z as i64))
        .fold((0,0,0), |(x, y, z), (x1, y1, z1)| (x.max(x1), y.max(y1), z.max(z1)));

    for (index, display_column) in data_vec.iter().enumerate() {
        if index == 1 || index == data_vec.len() -1{
            println!("{}", "-".repeat((data_all_len.0 + data_all_len.1 + data_all_len.2 + (LEN_PLUS *2)) as usize));
        }

        println!("{}{}{}{}{}", display_column.0,
                 " ".repeat(((data_all_len.0 - display_column.0.len() as i64) +LEN_PLUS) as usize),
                 display_column.1,
                 " ".repeat(((data_all_len.1 - display_column.1.len() as i64) +LEN_PLUS) as usize),
                 display_column.2
        )
    }

}

fn read_dir(p :&String) -> Vec<String> {
    let mut results :Vec<String> = Vec::new();
    let dir = PathBuf::from(p);
    if let Ok(files) = dir.read_dir() {
        for entry in files {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Some(path_string) = file_path.to_str() {
                    let path_string = path_string.to_string();
                    if file_path.is_dir() {
                        let s = read_dir(&path_string);
                        s.iter().for_each(|x| results.push(x.to_string()));
                    } else {
                        results.push(path_string);
                    }
                }
            }
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
