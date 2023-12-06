use std::collections::HashMap;

/*
  part 1 is solving
  speed * (time - hold) > distance
  Hold is the duration the button is hold
  Time is the duration the race last
  distance is the distance recorded for the race

  Also hold = speed so it becomes:
  speed * (time - speed) > distance
  or -speed^2 + speed * time > distance

  It's a quadratic equation. 
  The solutions are all the integers between the 2 roots of the equation.
*/

pub fn test_data() -> Vec<HashMap<&'static str, i64>> {
    /*
    Time:      7  15   30
    Distance:  9  40  200 
    */

    return vec![
        HashMap::<&str, i64>::from([
            ("time", 7),
            ("distance", 9)
        ]),
        HashMap::<&str, i64>::from([
            ("time", 15),
            ("distance", 40)
        ]),
        HashMap::<&str, i64>::from([
            ("time", 30),
            ("distance", 200)
        ])
    ];
}

pub fn data() -> Vec<HashMap<&'static str, i64>> {
    /*
    Time:        53     91     67     68
    Distance:   250   1330   1081   1025
    */

    return vec![
        HashMap::<&str, i64>::from([
            ("time", 53),
            ("distance", 250)
        ]),
        HashMap::<&str, i64>::from([
            ("time", 91),
            ("distance", 1330)
        ]),
        HashMap::<&str, i64>::from([
            ("time", 67),
            ("distance", 1081)
        ]),
        HashMap::<&str, i64>::from([
            ("time", 68),
            ("distance", 1025)
        ])
    ];
}

pub fn data2() -> (i64, i64) {
    return (53916768, 250133010811025)
}

pub fn get_roots(time: i64, distance: i64) -> (i64, i64) {
    /* eq: -speed^2 + speed * time - distance = 0 */
    let timef = time as f64;
    let distancef = (distance + 1) as f64; // +1 because it needs to be strictly greater than
    
    let root1 = 
        (-timef - (timef*timef - 4.0_f64 * -1.0_f64 * -distancef).sqrt())/
        (2.0_f64 * -1.0_f64);

    let root2 = 
        (-timef + (timef*timef - 4.0_f64 * -1.0_f64 * -distancef).sqrt())/
        (2.0_f64 * -1.0_f64);

    if root1 < root2 {
        return (root1.ceil() as i64, root2.floor() as i64)
    } else {
        return (root2.ceil() as i64, root1.floor() as i64)
    }

}