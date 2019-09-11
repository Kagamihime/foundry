extern crate foundry;

use foundry::view::View;
use foundry::Grid;

use std::fs;

#[test]
fn test_render() {
    let grid = Grid::from_file("tests/input_files/view_rendering.life").unwrap();

    let mut view = View::new(&grid);
    println!("Grid:\n{}", grid);

    view.set_position(3, 2);
    view.set_width(4);
    view.set_height(3);
    let rendering = view.render(view.width() * 2, view.height() * 2);
    println!("Rendering:\n{}", format_rendering(&rendering, &view));

    assert_eq!(0, rendering[0]);
    assert_eq!(0, rendering[1]);
    assert_eq!(255, rendering[2]);
    assert_eq!(255, rendering[3]);
    assert_eq!(255, rendering[4]);
    assert_eq!(255, rendering[5]);
    assert_eq!(255, rendering[6]);
    assert_eq!(255, rendering[7]);

    assert_eq!(0, rendering[8]);
    assert_eq!(0, rendering[9]);
    assert_eq!(255, rendering[10]);
    assert_eq!(255, rendering[11]);
    assert_eq!(255, rendering[12]);
    assert_eq!(255, rendering[13]);
    assert_eq!(255, rendering[14]);
    assert_eq!(255, rendering[15]);

    assert_eq!(0, rendering[16]);
    assert_eq!(0, rendering[17]);
    assert_eq!(0, rendering[18]);
    assert_eq!(0, rendering[19]);
    assert_eq!(255, rendering[20]);
    assert_eq!(255, rendering[21]);
    assert_eq!(0, rendering[22]);
    assert_eq!(0, rendering[23]);

    assert_eq!(0, rendering[24]);
    assert_eq!(0, rendering[25]);
    assert_eq!(0, rendering[26]);
    assert_eq!(0, rendering[27]);
    assert_eq!(255, rendering[28]);
    assert_eq!(255, rendering[29]);
    assert_eq!(0, rendering[30]);
    assert_eq!(0, rendering[31]);

    assert_eq!(0, rendering[32]);
    assert_eq!(0, rendering[33]);
    assert_eq!(0, rendering[34]);
    assert_eq!(0, rendering[35]);
    assert_eq!(255, rendering[36]);
    assert_eq!(255, rendering[37]);
    assert_eq!(0, rendering[38]);
    assert_eq!(0, rendering[39]);

    assert_eq!(0, rendering[40]);
    assert_eq!(0, rendering[41]);
    assert_eq!(0, rendering[42]);
    assert_eq!(0, rendering[43]);
    assert_eq!(255, rendering[44]);
    assert_eq!(255, rendering[45]);
    assert_eq!(0, rendering[46]);
    assert_eq!(0, rendering[47]);
}

fn format_rendering(r: &Vec<u8>, v: &View) -> String {
    let mut ret = String::new();
    let mut idx = 0;

    for _ in 0..v.height() * 2 {
        for _ in 0..v.width() * 2 {
            if r[idx] != 0 {
                ret.push_str("*");
            } else {
                ret.push_str(".");
            }

            idx += 1;
        }

        ret.push_str("\n");
    }

    ret
}
