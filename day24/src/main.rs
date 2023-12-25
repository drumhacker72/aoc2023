extern crate nalgebra as na;
use na::{Matrix2, Matrix6, Vector2, Vector3, Vector6};
use nom::character::complete::{char, i64, space1};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_vec3(s: &str) -> IResult<&str, Vector3<i64>> {
    let (s, (x, _, _, y, _, _, z)) =
        tuple((i64, char(','), space1, i64, char(','), space1, i64))(s)?;
    Ok((s, Vector3::new(x, y, z)))
}

fn parse_line(s: &str) -> IResult<&str, (Vector3<i64>, Vector3<i64>)> {
    separated_pair(parse_vec3, delimited(space1, char('@'), space1), parse_vec3)(s)
}

fn intersection_2d(
    p_a: &Vector3<i64>,
    v_a: &Vector3<i64>,
    p_b: &Vector3<i64>,
    v_b: &Vector3<i64>,
) -> Option<Vector2<f64>> {
    let p_a = p_a.xy().cast::<f64>();
    let v_a = v_a.xy().cast::<f64>();
    let p_b = p_b.xy().cast::<f64>();
    let v_b = v_b.xy().cast::<f64>();
    // p_z = p_a + v_a * t_a
    //     = p_b + v_b * t_b
    // p_z.x = p_a.x + v_a.x * t_a
    //       = p_b.x + v_b.x * t_b
    // p_z.y = p_a.y + v_a.y * t_a
    //       = p_b.y + v_b.y * t_b
    let mut m = Matrix2::from_columns(&[v_a, -v_b]);
    if !m.try_inverse_mut() {
        return None;
    }
    let t = m * (p_b - p_a);
    if t[0] < 0.0 || t[1] < 0.0 {
        return None;
    }
    Some(p_a + v_a * t[0])
}

fn main() {
    let file = File::open("day24.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let hailstones: Vec<(Vector3<i64>, Vector3<i64>)> = lines
        .map(|line| {
            let (remaining, (p, v)) = parse_line(&line).unwrap();
            assert!(remaining == "");
            (p, v)
        })
        .collect();
    let mut count = 0;
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    for i in 0..hailstones.len() {
        for j in 0..i {
            let a = &hailstones[i];
            let b = &hailstones[j];
            if let Some(r) = intersection_2d(&a.0, &a.1, &b.0, &b.1) {
                if MIN <= r.x && r.x <= MAX && MIN <= r.y && r.y <= MAX {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");

    // finding thrown hailstone Z, given hailstones A, B, C, ...
    // pz + vz * ta = pa + va * ta
    // (pz - pa) = ta (va - vz)
    // Since (pz - pa) and (va - vz) are in the same direction, their cross-product is 0,
    // so can eliminate the time variable:
    // (pz - pa) x (va - vz) = 0
    // (pz x va) - (pa x va) - (pz x vz) + (pa x vz) = 0
    // pz x vz = (pz x va) - (pa x va) + (pa x vz)
    //         = (pz x vb) - (pb x vb) + (pb x vz)
    //         = ...
    //   (pz.y * va.z - pz.z * va.y) - (pa.y * va.z - pa.z * va.y) + (pa.y * vz.z - pa.z * vz.y)
    // = (pz.y * vb.z - pz.z * vb.y) - (pb.y * vb.z - pb.z * vb.y) + (pb.y * vz.z - pb.z * vz.y)
    // for the x-coordinate, etc for y and z
    // ... and then again for a different pair C and D (or even A/C) of given hailstones.
    // 6 unknowns, 6 equations, linear system.
    // [0 (va.z-vb.z) (vb.y-va.y) 0 (pb.z-pa.z) (pa.y-pb.y)] [pz.x]   [pb.y*vb.z - pb.z*vb.y - pa.y*va.z + pa.z*va.y]
    // [                       ...                         ] [pz.y]   [          ...                                ]
    // [                       ...                         ] [pz.z] = [          ...                                ]
    // [                       ...                         ] [vz.x]   [          ...                                ]
    // [                       ...                         ] [vz.y]   [          ...                                ]
    // [                       ...                         ] [vz.z]   [          ...                                ]
    let pa = &hailstones[0].0.cast::<f64>();
    let va = &hailstones[0].1.cast::<f64>();
    let pb = &hailstones[1].0.cast::<f64>();
    let vb = &hailstones[1].1.cast::<f64>();
    let pc = &hailstones[2].0.cast::<f64>();
    let vc = &hailstones[2].1.cast::<f64>();
    let mut m = Matrix6::zeros();
    m.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&(va.cross_matrix() - vb.cross_matrix()));
    m.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&(pb.cross_matrix() - pa.cross_matrix()));
    m.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&(va.cross_matrix() - vc.cross_matrix()));
    m.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&(pc.cross_matrix() - pa.cross_matrix()));
    assert!(m.try_inverse_mut());
    let mut v = Vector6::zeros();
    v.fixed_view_mut::<3, 1>(0, 0)
        .copy_from(&(pb.cross(vb) - pa.cross(va)));
    v.fixed_view_mut::<3, 1>(3, 0)
        .copy_from(&(pc.cross(vc) - pa.cross(va)));
    let r = m * v;
    let p = r.xyz().map(|v| v.round()).try_cast::<i64>().unwrap();
    println!("{}", p.x + p.y + p.z);
}
