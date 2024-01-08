use anyhow;
use aoc23;
use itertools::*;
use ndarray::prelude::*;
use ndarray_linalg::{error::LinalgError, Solve};

#[derive(Debug, Clone)]
struct Hailstone {
    id: usize,
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Hailstone {
    fn from_line(id: usize, line: &str) -> Hailstone {
        let (pos, vel) = line.split_once('@').unwrap();
        let (x, y, z) = pos
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let pos = [x, y, z];
        let (x, y, z) = vel
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let vel = [x, y, z];
        Hailstone { id, pos, vel }
    }

    fn pos_f64(&self) -> Array1<f64> {
        v_i64_to_arr1_f64(&self.pos)
    }

    fn vel_f64(&self) -> Array1<f64> {
        v_i64_to_arr1_f64(&self.vel)
    }
}

fn v_i64_to_arr1_f64(v: &[i64; 3]) -> Array1<f64> {
    Array1::from_iter(v.iter().map(|&i| i as f64))
}

fn a_i(h: &Hailstone) -> Array2<f64> {
    /*
                p_s_1    p_s_2    p_s_3   v_s_1    v_s_2    v_s_3
            -------------------------------------------------
            |      0   -v_i_3    v_i_2       0    p_i_3   -p_i_2 |
    A_i =   |  v_i_3        0   -v_i_1  -p_i_3        0    p_i_1 |
            | -v_i_2    v_i_1        0   p_i_2   -p_i_1        0 |

        Note while implementing the axis need to be mapped
        from 1,2,3 to 0,1,2 of array index
    */

    let v_i = h.vel_f64();
    let p_i = h.pos_f64();

    let a_i = arr2(&[
        [0.0, -v_i[2], v_i[1], 0.0, p_i[2], -p_i[1]],
        [v_i[2], 0.0, -v_i[0], -p_i[2], 0.0, p_i[0]],
        [-v_i[1], v_i[0], 0.0, p_i[1], -p_i[0], 0.0],
    ]);

    a_i
}

fn b_i(h: &Hailstone) -> Array1<f64> {
    /*
    b_i =   | - v_i_2*p_i_3 + v_i_3*p_i_2 |
            | - v_i_3*p_i_1 + v_i_1*p_i_3 |
            | - v_i_1*p_i_2 + v_i_2*p_i_1 |

    b_i = - v_i cross p_i

        Note while implementing the axis need to be mapped
        from 1,2,3 to 0,1,2 of array index
     */

    let v_i = h.vel_f64();
    let p_i = h.pos_f64();

    let b_i = arr1(&[
        -v_i[1] * p_i[2] + v_i[2] * p_i[1],
        -v_i[2] * p_i[0] + v_i[0] * p_i[2],
        -v_i[0] * p_i[1] + v_i[1] * p_i[0],
    ]);

    b_i
}

fn main() -> anyhow::Result<()> {
    let lines = aoc23::read_input_lines();
    let stones = lines
        .enumerate()
        .map(|(id, s)| Hailstone::from_line(id, &s))
        .collect_vec();

    /*
    For the overall eqation system we must solve for x

    | (A_1 - A_2)x = b_2 - b_1 |
    | (A_1 - A_3)x = b_3 - b_1 |

        for any 3 stones
     */

    // hand picked to be correct
    let h_1 = &stones[0];
    let h_2 = &stones[27];
    let h_3 = &stones[290];

    let a_1 = a_i(h_1);
    let a_2 = a_i(h_2);
    let a_3 = a_i(h_3);

    let b_1 = b_i(h_1);
    let b_2 = b_i(h_2);
    let b_3 = b_i(h_3);

    let a_upper = a_1.clone() - a_2;
    let a_lower = a_1.clone() - a_3;
    let b_upper = b_2 - b_1.clone();
    let b_lower = b_3 - b_1.clone();

    let mut a_ = a_upper.clone();
    a_.append(Axis(0), (&a_lower).into()).unwrap();
    let mut b = b_upper.clone();
    b.append(Axis(0), (&b_lower).into()).unwrap();

    let x = a_.solve_into(b).unwrap();

    /*
    x = | p_s_1 |
        | p_s_2 |
        | p_s_3 |
        | v_s_1 |
        | v_s_2 |
        | v_s_3 |
    */

    println!("x = {:?}", &x);

    let sum = x[0] + x[1] + x[2];
    println!("{}", sum);
    let sum = sum as i64;
    println!("{}", sum);
    Ok(())
}
