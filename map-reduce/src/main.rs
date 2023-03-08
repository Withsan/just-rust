use std::slice::Chunks;
use std::thread;
use std::time::{SystemTime, SystemTimeError};

const THREAD_NUM: usize = 10;

fn main() -> Result<(), SystemTimeError> {
    let start = SystemTime::now();
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668"
        .repeat(100000);
    let str_data = data.as_str();
    let result = get_sum_1(str_data);
    let end = SystemTime::now();
    println!(
        "sum is {:?},and cost {:?}",
        result.unwrap(),
        end.duration_since(start)
    );
    Ok(())
}
#[allow(unused)]
fn get_sum_1(str: &str) -> Result<u32, SystemTimeError> {
    let chunk_size = get_chunk_size(str);
    let vec = chunks(str, chunk_size);
    reduce_1(vec)
}

#[allow(unused)]
fn reduce_1(chunked_data: Vec<&str>) -> Result<u32, SystemTimeError> {
    let scoped = thread::scope(|s| {
        let mut children = vec![];
        chunked_data.into_iter().for_each(|item| {
            children.push(s.spawn(|| -> u32 {
                let result = item
                    .chars()
                    .filter_map(|c| c.to_digit(10).or(Some(0)))
                    .sum();
                result
            }));
        });
        children.into_iter().map(|c| c.join().unwrap()).sum::<u32>()
    });
    Ok(scoped)
}
#[allow(unused)]
fn chunks(str: &str, chunked_size: usize) -> Vec<&str> {
    let mut result = vec![];
    let split = str.split_at(str.len() / 2);
    if split.0.len() < chunked_size || split.1.len() < chunked_size {
        result.push(split.0);
        result.push(split.1);
    } else {
        result.append(&mut chunks(split.0, chunked_size));
        result.append(&mut chunks(split.1, chunked_size));
    }
    result
}
#[allow(unused)]
fn get_sum_2(str: &str) -> Result<u32, SystemTimeError> {
    let chunk_size = get_chunk_size(str);
    let split = str.split("");
    let split_data = split.collect::<Vec<&str>>();
    let chunked_data = split_data.chunks(chunk_size);
    reduce_2(chunked_data)
}

#[allow(unused)]
fn reduce_2(chunked_data: Chunks<&str>) -> Result<u32, SystemTimeError> {
    let mut children = vec![];
    chunked_data.into_iter().for_each(|item| {
        let to_sum = item.join("");
        children.push(thread::spawn(move || -> u32 {
            let result = to_sum.chars().filter_map(|c| c.to_digit(10)).sum();
            result
        }));
    });
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    Ok(final_result)
}
#[allow(unused)]
fn get_sum_3(str: &str) -> Result<u32, SystemTimeError> {
    let chunk_size = get_chunk_size(str);
    let split = str.split("");
    let split_data = split.collect::<Vec<&str>>();
    let chunked_data = split_data.chunks(chunk_size);
    reduce_3(chunked_data)
}

#[allow(unused)]
fn reduce_3(chunked_data: Chunks<&str>) -> Result<u32, SystemTimeError> {
    let scoped = thread::scope(|s| {
        let mut children = vec![];
        chunked_data.into_iter().for_each(|item| {
            children.push(s.spawn(|| -> u32 {
                let result = item.iter().filter_map(|c| c.parse::<u32>().ok()).sum();
                result
            }));
        });
        children.into_iter().map(|c| c.join().unwrap()).sum::<u32>()
    });
    Ok(scoped)
}

fn get_chunk_size(data: &str) -> usize {
    let data_len = data.len();
    if data_len > 100000 {
        return data_len / THREAD_NUM;
    }
    data_len
}

#[cfg(test)]
mod tests {

    use crate::get_sum_1;
    use std::time::SystemTime;

    #[test]
    fn test_get_sum_1() {
        let start = SystemTime::now();
        let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668"
            .repeat(10000);
        let str_data = data.as_str();
        let result = get_sum_1(str_data);
        let end = SystemTime::now();
        println!(
            "sum1 is {:?},and cost {:?}",
            result.unwrap(),
            end.duration_since(start)
        );
    }
    #[test]
    fn test_get_sum_2() {
        let start = SystemTime::now();
        let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668"
            .repeat(10000);
        let str_data = data.as_str();
        let result = get_sum_1(str_data);
        let end = SystemTime::now();
        println!(
            "sum2 is {:?},and cost {:?}",
            result.unwrap(),
            end.duration_since(start)
        );
    }
    #[test]
    fn test_get_sum_3() {
        let start = SystemTime::now();
        let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668"
            .repeat(10000);
        let str_data = data.as_str();
        let result = get_sum_1(str_data);
        let end = SystemTime::now();
        println!(
            "sum3 is {:?},and cost {:?}",
            result.unwrap(),
            end.duration_since(start)
        );
    }
}
