use bitpacking::{fastpack, fastunpack};

static TEST_DATA: [u32; 32] = [
    1506, 468, 3129, 2824, 1715, 3459, 448, 1685, 242, 3189, 1405, 1689, 2603, 1459, 2860, 2397,
    4019, 823, 464, 123, 2422, 1142, 1492, 3915, 2152, 2890, 662, 2045, 3823, 739, 3650, 326
];

// First 11 Tests should panic as the largest number in `TEST_DATA` is 4019 (which is 12 bit big)

#[test]
fn fastunpack0() {
    let mut output: [u32; 32] = [0; 32];
    let temp: [u32; 32] = [0; 32];

    fastunpack(&temp, 0, &mut output, 0, 1);

    assert_eq!(temp, output);
}

#[test]
#[should_panic]
fn fastunpack1() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 1);
    fastunpack(&temp, 0, &mut output, 0, 1);

    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack2() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 2);
    fastunpack(&temp, 0, &mut output, 0, 2);

    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack3() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 3);
    fastunpack(&temp, 0, &mut output, 0, 3);

    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack4() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 4);
    fastunpack(&temp, 0, &mut output, 0, 4);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack5() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 5);
    fastunpack(&temp, 0, &mut output, 0, 5);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack6() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 6);
    fastunpack(&temp, 0, &mut output, 0, 6);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack7() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 7);
    fastunpack(&temp, 0, &mut output, 0, 7);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack8() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 8);
    fastunpack(&temp, 0, &mut output, 0, 8);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack9() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 9);
    fastunpack(&temp, 0, &mut output, 0, 9);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack10() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 10);
    fastunpack(&temp, 0, &mut output, 0, 10);


    assert_eq!(output, TEST_DATA);
}

#[test]
#[should_panic]
fn fastunpack11() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 11);
    fastunpack(&temp, 0, &mut output, 0, 11);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack12() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 12);
    fastunpack(&temp, 0, &mut output, 0, 12);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack13() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 13);
    fastunpack(&temp, 0, &mut output, 0, 13);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack14() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 14);
    fastunpack(&temp, 0, &mut output, 0, 14);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack15() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 15);
    fastunpack(&temp, 0, &mut output, 0, 15);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack16() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 16);
    fastunpack(&temp, 0, &mut output, 0, 16);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack17() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 17);
    fastunpack(&temp, 0, &mut output, 0, 17);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack18() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 18);
    fastunpack(&temp, 0, &mut output, 0, 18);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack19() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 19);
    fastunpack(&temp, 0, &mut output, 0, 19);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack20() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 20);
    fastunpack(&temp, 0, &mut output, 0, 20);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack21() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 21);
    fastunpack(&temp, 0, &mut output, 0, 21);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack22() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 22);
    fastunpack(&temp, 0, &mut output, 0, 22);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack23() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 23);
    fastunpack(&temp, 0, &mut output, 0, 23);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack24() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 24);
    fastunpack(&temp, 0, &mut output, 0, 24);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack25() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 25);
    fastunpack(&temp, 0, &mut output, 0, 25);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack26() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 26);
    fastunpack(&temp, 0, &mut output, 0, 26);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack27() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 27);
    fastunpack(&temp, 0, &mut output, 0, 27);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack28() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 28);
    fastunpack(&temp, 0, &mut output, 0, 28);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack29() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 29);
    fastunpack(&temp, 0, &mut output, 0, 29);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack30() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 30);
    fastunpack(&temp, 0, &mut output, 0, 30);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack31() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 31);
    fastunpack(&temp, 0, &mut output, 0, 31);


    assert_eq!(output, TEST_DATA);
}

#[test]
fn fastunpack32() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    fastpack(&TEST_DATA, 0, &mut temp, 0, 32);
    fastunpack(&temp, 0, &mut output, 0, 32);


    assert_eq!(output, TEST_DATA);
    assert_eq!(output, TEST_DATA); // Equals
}

#[test]
#[should_panic]
fn fastunpack33() {
    let mut output: [u32; 32] = [0; 32];
    let mut temp: [u32; 32] = [0; 32];

    // 33 bitwidth is not supported, please panic
    fastpack(&TEST_DATA, 0, &mut temp, 0, 33);
    fastunpack(&temp, 0, &mut output, 0, 33);

}
