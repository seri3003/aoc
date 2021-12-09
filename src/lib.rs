use std::io::Read;

mod aoc;

fn measure_depth(depth: &[u32]) -> u32 {
    let deep = depth.iter();
    let mut counter = 0;
    let val = depth.iter().zip(deep.skip(1)).map(|(cur, ne)| if cur < ne { counter +=1 } ).count();
    counter 
}
fn measure_depth_window(depth: &[u32]) -> u32 {
    let deep = depth.iter();
    let mut summed_val : Vec<u32> = Vec::new();
    while depth.iter().next().is_some(){
        let val = deep.take(3).sum::<u32>();
        summed_val.push(val);
    }
    measure_depth(&summed_val)
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::aoc::day1;

    #[test]
    fn test_measure_depth() {
        let simple_test = [199,200,208,210,200,207,240,269,260,263];
        let res =  measure_depth(&simple_test);
        assert_eq!(res,7);
    }
    #[test]
    fn test_measure_depth_real() {
        let res =  measure_depth(day1::MEASUREMENTS);
        assert_eq!(res,7);
    }
    #[test]
    fn test_measure_depth_window() {
        let simple_test = [199,200,208,210,200,207,240,269,260,263];
        let res =  measure_depth_window(&simple_test);
        assert_eq!(res,7);
    }
}
