
mod aoc;

/// Calculates the changes in depth if depth in increased 
pub fn measure_depth(depth: &[u32]) -> u32 {
    let deep = depth.iter();
    let mut counter = 0;
    depth.iter().zip(deep.skip(1)).map(|(cur, ne)| if cur < ne { counter +=1 } ).count();
    counter 
}


/// Applies a windowed with a windowsize of 3 to the meassured depth
pub fn measure_depth_window(depth: &[u32]) -> u32 {
    let mut summed_val : Vec<u32> = Vec::new();

    depth.windows(3).map(|x|summed_val.push(x.iter().sum())).count();

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
        assert_eq!(res,1448);
    }
    #[test]
    fn test_measure_depth_window() {
        let simple_test = [199,200,208,210,200,207,240,269,260,263];
        let res =  measure_depth_window(&simple_test);
        assert_eq!(res,5);
    }
    #[test]
    fn test_measure_depth_window_real() {
        let res =  measure_depth_window(&day1::MEASUREMENTS);
        assert_eq!(res,1471);
    }
}
